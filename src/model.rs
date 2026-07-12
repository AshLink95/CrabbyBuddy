use llama_cpp_2::context::LlamaContext;
use llama_cpp_2::llama_backend::LlamaBackend;
use llama_cpp_2::llama_batch::LlamaBatch;
use llama_cpp_2::model::{AddBos, LlamaChatMessage, LlamaModel, params::LlamaModelParams};
use llama_cpp_2::context::params::LlamaContextParams;
use llama_cpp_2::sampling::LlamaSampler;
use anyhow::Result;
use llama_cpp_2::token::LlamaToken;
use std::fs::read_dir;
use std::num::NonZeroU32;

#[derive(PartialEq, Eq)]
enum ChatFormat { Native, Gemma }
pub struct Model {
    id: i32,
    model: &'static LlamaModel,
    cntx: LlamaContext<'static>,
    tokens: Vec<LlamaToken>,
    pos: i32,
    format: ChatFormat,
}

fn load(model_idx: usize) -> Result<(&'static LlamaModel, LlamaContext<'static>, ChatFormat)> {
    llama_cpp_2::send_logs_to_tracing(llama_cpp_2::LogOptions::default());
    let backend = Box::leak(Box::new(LlamaBackend::init()?));
    let models: Vec<String> = read_dir(
            dirs::data_local_dir().unwrap().join("crabbybuddy/models")
        )?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.is_file() && p.extension().is_some_and(|x| x == "gguf"))
        .map(|e| e.to_string_lossy().into_owned())
        .collect();
    let model_path = &models[model_idx];
    let format = if model_path.to_lowercase().contains("gemma") {
        ChatFormat::Gemma
    } else { ChatFormat::Native };
    let model = Box::leak(Box::new(LlamaModel::load_from_file(
        backend, model_path.trim(),
        //TODO: load gpu layers based on model meta data
        &LlamaModelParams::default().with_n_gpu_layers(u32::MAX),
    )?));
    let cntx = model.new_context( backend,
        LlamaContextParams::default().with_n_ctx(NonZeroU32::new(4096)),
    )?;

    Ok((model, cntx, format))
}

impl Model {
    pub fn load_new(id: i32, model_idx: usize, sys_prompt: &str) -> Result<Self> {
        let (model, mut cntx, format) = load(model_idx)?;

        let system_prompt = String::from(sys_prompt);
        let tmpl = model.chat_template(None)?;
        let tokens = if format == ChatFormat::Native {
            let messages = vec![
                LlamaChatMessage::new("system".into(), system_prompt)?,
            ];
            let templated = model.apply_chat_template(&tmpl, &messages, true)?;
            model.str_to_token(&templated, AddBos::Always)?
        } else {
            let templated = format!("<|turn>system\n{}<turn|>\n", system_prompt);
            model.str_to_token(&templated, AddBos::Never)?
        };
        let mut batch = LlamaBatch::new(512, 1);
        for (i, &tok) in tokens.iter().enumerate() {
            batch.add(tok, i as i32, &[0], i == tokens.len() - 1)?;
        }
        cntx.decode(&mut batch)?;
        let pos = tokens.len() as i32;

        if id != 0 {
            //TODO: save plain convo and dump on reload for cross model sessions
            let session_path = dirs::data_local_dir().unwrap().join(
                format!("crabbybuddy/sessions/crabby{id}.session")
            );
            cntx.state_save_file(session_path, &tokens)?;
        }

        Ok(Self { id, model, cntx, tokens, pos, format })
    }

    pub fn load_old(id: i32, model_idx: usize) -> Result<Self> {
        let (model, mut cntx, format) = load(model_idx)?;

        let session_path = dirs::data_local_dir().unwrap()
            .join(format!("crabbybuddy/sessions/crabby{id}.session"));

        let tokens = cntx.state_load_file(&session_path, 4096)?;
        let pos = tokens.len() as i32;

        Ok(Self { id, model, cntx, tokens, pos, format })
    }

    pub fn chat(&mut self, prompt: &str) -> Result<String> {
        let templated = if self.format == ChatFormat::Native {
            let tmpl = self.model.chat_template(None)?;
            let msgs = vec![LlamaChatMessage::new("user".into(), prompt.into())?];
            self.model.apply_chat_template(&tmpl, &msgs, true)?
        } else {
            format!("<|turn>user\n{prompt}<turn|>\n<|turn>model\n")
        };
        let tokens = self.model.str_to_token(&templated, AddBos::Never)?;

        let mut batch = LlamaBatch::new(512, 1);
        let last = tokens.len() - 1;
        for (i, &tok) in tokens.iter().enumerate() {
            batch.add(tok, self.pos + i as i32, &[0], i == last)?;
        }
        self.tokens.extend_from_slice(&tokens);
        self.pos += tokens.len() as i32;
        self.cntx.decode(&mut batch)?;

        let mut sampler = LlamaSampler::chain_simple([LlamaSampler::greedy()]);
        let mut bytes = Vec::new();
        for _ in 0..1024 {
            let tok = sampler.sample(&self.cntx, -1);
            if self.model.is_eog_token(tok) { break; }
            bytes.extend_from_slice(
                &self.model.token_to_piece_bytes(tok, 64, false, None)?
            );
            self.tokens.push(tok);
            batch.clear();
            batch.add(tok, self.pos, &[0], true)?;
            self.pos += 1;
            self.cntx.decode(&mut batch)?;
        }

        if self.id != 0 {
            let session_path = dirs::data_local_dir().unwrap().join(
                format!("crabbybuddy/sessions/crabby{}.session", self.id)
            );
            self.cntx.state_save_file(session_path, &self.tokens)?;
        }

        Ok(String::from_utf8_lossy(&bytes).into_owned())
    }
}
