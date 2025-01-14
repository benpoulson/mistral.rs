# mistral.rs PyO3 Bindings: `mistralrs`

`mistralrs` is a Python package which provides an API for `mistral.rs`. We build `mistralrs` with the `maturin` build manager.

## Installation
1) Install required packages
    - `libssl` (ex., `sudo apt install libssl-dev`)
    - `pkg-config` (ex., `sudo apt install pkg-config`)

2) Install Rust: https://rustup.rs/
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source $HOME/.cargo/env
    ```

3) Set HF token correctly (skip if already set or your model is not gated, or if you want to use the `token_source` parameters in Python or the command line.)
    ```bash
    mkdir ~/.cache/huggingface
    touch ~/.cache/huggingface/token
    echo <HF_TOKEN_HERE> > ~/.cache/huggingface/token
    ```

4) Download the code
    ```bash
    git clone https://github.com/EricLBuehler/mistral.rs.git
    cd mistral.rs
    ```

5) `cd` into the correct directory for building `mistralrs`:
    `cd mistralrs-pyo3`

6) Install `maturin`, our Rust + Python build system:
    Maturin requires a Python virtual environment such as `venv` or `conda` to be active. The `mistralrs` package will be installed into that
    environment.
    ```
    pip install maturin[patchelf]
    ```

7) Install `mistralrs`
    Install `mistralrs` by executing the following in this directory where [features](../README.md#supported-accelerators) such as `cuda` or `flash-attn` may be specified with the `--features` argument just like they would be for `cargo run`.

    The base build command is:
    ```bash
    maturin develop -r
    ```

    - To build for CUDA:
    
    ```bash
    maturin develop -r --features cuda
    ```
    
    - To build for CUDA with flash attention:
    
    ```bash
    maturin develop -r --features "cuda flash-attn"
    ```

    - To build for Metal:  

    ```bash
    maturin develop -r --features metal
    ```

    - To build for Accelerate:  
      
    ```bash
    maturin develop -r --features accelerate
    ```

    - To build for MKL:  
      
    ```bash
    maturin develop -r --features mkl
    ```

Please find [API docs here](API.md) and the type stubs [here](mistralrs.pyi), which are another great form of documentation.

We also provide [a cookbook here](../examples/python/cookbook.ipynb)!

## Example
```python
from mistralrs import ModelKind, MistralLoader, ChatCompletionRequest

kind = ModelKind.QuantizedGGUF
loader = MistralLoader(
    model_id="mistralai/Mistral-7B-Instruct-v0.1",
    kind=kind,
    no_kv_cache=False,
    repeat_last_n=64,
    quantized_model_id="TheBloke/Mistral-7B-Instruct-v0.1-GGUF",
    quantized_filename="mistral-7b-instruct-v0.1.Q4_K_M.gguf",
)
runner = loader.load()
res = runner.send_chat_completion_request(
    ChatCompletionRequest(
        model="mistral",
        messages=[
            {"role": "user", "content": "Tell me a story about the Rust type system."}
        ],
        max_tokens=256,
        frequency_penalty=1.0,
        top_p=0.1,
        temperature=0.1,
    )
)
print(res)
```