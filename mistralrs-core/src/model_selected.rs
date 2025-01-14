use clap::Subcommand;

use crate::pipeline::NormalLoaderType;

fn parse_arch(x: &str) -> Result<NormalLoaderType, String> {
    x.parse()
}

#[derive(Debug, Subcommand)]
pub enum ModelSelected {
    /// Select a plain model
    Plain {
        /// Model ID to load from
        #[arg(short, long)]
        model_id: String,

        /// Path to local tokenizer.json file. If this is specified it is used over any remote file.
        #[arg(short, long)]
        tokenizer_json: Option<String>,

        /// Control the application of repeat penalty for the last n tokens
        #[arg(long, default_value_t = 64)]
        repeat_last_n: usize,

        #[arg(short, long, value_parser = parse_arch)]
        arch: NormalLoaderType,
    },

    /// Select an X-LoRA architecture
    XLora {
        /// Force a base model ID to load from instead of using the ordering file.
        #[arg(short, long)]
        model_id: Option<String>,

        /// Path to local tokenizer.json file. If this is specified it is used over any remote file.
        #[arg(short, long)]
        tokenizer_json: Option<String>,

        /// Model ID to load X-LoRA from.
        #[arg(short, long)]
        xlora_model_id: String,

        /// Control the application of repeat penalty for the last n tokens
        #[arg(long, default_value_t = 64)]
        repeat_last_n: usize,

        /// Ordering JSON file
        #[arg(short, long)]
        order: String,

        /// Index of completion tokens to generate scalings up until. If this is 1, then there will be one completion token generated before it is cached.
        /// This makes the maximum running sequences 1.
        #[arg(long)]
        tgt_non_granular_index: Option<usize>,

        #[arg(short, long, value_parser = parse_arch)]
        arch: NormalLoaderType,
    },

    /// Select a LoRA architecture
    Lora {
        /// Force a base model ID to load from instead of using the ordering file.
        #[arg(short, long)]
        model_id: Option<String>,

        /// Path to local tokenizer.json file. If this is specified it is used over any remote file.
        #[arg(short, long)]
        tokenizer_json: Option<String>,

        /// Model ID to load X-LoRA from.
        #[arg(short, long)]
        adapters_model_id: String,

        /// Control the application of repeat penalty for the last n tokens
        #[arg(long, default_value_t = 64)]
        repeat_last_n: usize,

        /// Ordering JSON file
        #[arg(short, long)]
        order: String,

        #[arg(short, long, value_parser = parse_arch)]
        arch: NormalLoaderType,
    },

    /// Select a GGUF model.
    GGUF {
        /// Model ID to load the tokenizer from
        #[arg(short, long)]
        tok_model_id: String,

        /// Path to local tokenizer.json file. If this is specified it is used over any remote file.
        #[arg(long)]
        tokenizer_json: Option<String>,

        /// Quantized model ID to find the `quantized_filename`, only applicable if `quantized` is set.
        #[arg(short = 'm', long)]
        quantized_model_id: String,

        /// Quantized filename, only applicable if `quantized` is set.
        #[arg(short = 'f', long)]
        quantized_filename: String,

        /// Control the application of repeat penalty for the last n tokens
        #[arg(long, default_value_t = 64)]
        repeat_last_n: usize,
    },

    /// Select a GGUF model with X-LoRA.
    XLoraGGUF {
        /// Model ID to load the tokenizer from
        #[arg(short, long)]
        tok_model_id: Option<String>,

        /// Path to local tokenizer.json file. If this is specified it is used over any remote file.
        #[arg(long)]
        tokenizer_json: Option<String>,

        /// Quantized model ID to find the `quantized_filename`, only applicable if `quantized` is set.
        #[arg(short = 'm', long)]
        quantized_model_id: String,

        /// Quantized filename, only applicable if `quantized` is set.
        #[arg(short = 'f', long)]
        quantized_filename: String,

        /// Control the application of repeat penalty for the last n tokens
        #[arg(long, default_value_t = 64)]
        repeat_last_n: usize,

        /// Model ID to load X-LoRA from.
        #[arg(short, long)]
        xlora_model_id: String,

        /// Ordering JSON file
        #[arg(short, long)]
        order: String,

        /// Index of completion tokens to generate scalings up until. If this is 1, then there will be one completion token generated before it is cached.
        /// This makes the maximum running sequences 1.
        #[arg(long)]
        tgt_non_granular_index: Option<usize>,
    },

    /// Select a GGUF model with LoRA.
    LoraGGUF {
        /// Model ID to load the tokenizer from
        #[arg(short, long)]
        tok_model_id: Option<String>,

        /// Path to local tokenizer.json file. If this is specified it is used over any remote file.
        #[arg(long)]
        tokenizer_json: Option<String>,

        /// Quantized model ID to find the `quantized_filename`, only applicable if `quantized` is set.
        #[arg(short = 'm', long)]
        quantized_model_id: String,

        /// Quantized filename, only applicable if `quantized` is set.
        #[arg(short = 'f', long)]
        quantized_filename: String,

        /// Control the application of repeat penalty for the last n tokens
        #[arg(long, default_value_t = 64)]
        repeat_last_n: usize,

        /// Model ID to load X-LoRA from.
        #[arg(short, long)]
        adapters_model_id: String,

        /// Ordering JSON file
        #[arg(short, long)]
        order: String,

        /// Index of completion tokens to generate scalings up until. If this is 1, then there will be one completion token generated before it is cached.
        /// This makes the maximum running sequences 1.
        #[arg(long)]
        tgt_non_granular_index: Option<usize>,
    },

    /// Select a GGML model.
    GGML {
        /// Model ID to load the tokenizer from
        #[arg(short, long)]
        tok_model_id: String,

        /// Path to local tokenizer.json file. If this is specified it is used over any remote file.
        #[arg(long)]
        tokenizer_json: Option<String>,

        /// Quantized model ID to find the `quantized_filename`, only applicable if `quantized` is set.
        #[arg(short = 'm', long)]
        quantized_model_id: String,

        /// Quantized filename, only applicable if `quantized` is set.
        #[arg(short = 'f', long)]
        quantized_filename: String,

        /// Control the application of repeat penalty for the last n tokens
        #[arg(long, default_value_t = 64)]
        repeat_last_n: usize,

        /// GQA value
        #[arg(short, long, default_value_t = 1)]
        gqa: usize,
    },

    /// Select a GGML model with X-LoRA.
    XLoraGGML {
        /// Model ID to load the tokenizer from
        #[arg(short, long)]
        tok_model_id: Option<String>,

        /// Path to local tokenizer.json file. If this is specified it is used over any remote file.
        #[arg(long)]
        tokenizer_json: Option<String>,

        /// Quantized model ID to find the `quantized_filename`, only applicable if `quantized` is set.
        #[arg(short = 'm', long)]
        quantized_model_id: String,

        /// Quantized filename, only applicable if `quantized` is set.
        #[arg(short = 'f', long)]
        quantized_filename: String,

        /// Control the application of repeat penalty for the last n tokens
        #[arg(long, default_value_t = 64)]
        repeat_last_n: usize,

        /// Model ID to load X-LoRA from.
        #[arg(short, long)]
        xlora_model_id: String,

        /// Ordering JSON file
        #[arg(short, long)]
        order: String,

        /// Index of completion tokens to generate scalings up until. If this is 1, then there will be one completion token generated before it is cached.
        /// This makes the maximum running sequences 1.
        #[arg(long)]
        tgt_non_granular_index: Option<usize>,

        /// GQA value
        #[arg(short, long, default_value_t = 1)]
        gqa: usize,
    },

    /// Select a GGML model with LoRA.
    LoraGGML {
        /// Model ID to load the tokenizer from
        #[arg(short, long)]
        tok_model_id: Option<String>,

        /// Path to local tokenizer.json file. If this is specified it is used over any remote file.
        #[arg(long)]
        tokenizer_json: Option<String>,

        /// Quantized model ID to find the `quantized_filename`, only applicable if `quantized` is set.
        #[arg(short = 'm', long)]
        quantized_model_id: String,

        /// Quantized filename, only applicable if `quantized` is set.
        #[arg(short = 'f', long)]
        quantized_filename: String,

        /// Control the application of repeat penalty for the last n tokens
        #[arg(long, default_value_t = 64)]
        repeat_last_n: usize,

        /// Model ID to load X-LoRA from.
        #[arg(short, long)]
        adapters_model_id: String,

        /// Ordering JSON file
        #[arg(short, long)]
        order: String,

        /// Index of completion tokens to generate scalings up until. If this is 1, then there will be one completion token generated before it is cached.
        /// This makes the maximum running sequences 1.
        #[arg(long)]
        tgt_non_granular_index: Option<usize>,

        /// GQA value
        #[arg(short, long, default_value_t = 1)]
        gqa: usize,
    },
}
