import os
os.environ["RADV_PERFMODE"] = "nogttspill"
os.environ["VLLM_WORKER_MULTIPROC_METHOD"] = "spawn"
os.environ["VLLM_USE_V1"] = "0"

def main():
    from vllm import LLM, SamplingParams
    print("Loading LongCat-Next-INT4 into vLLM (V0 Engine)...")
    try:
        llm = LLM(
            model="/home/joshua/trinity-models/omni/LongCat-Next-INT4",
            trust_remote_code=True,
            tensor_parallel_size=1,
            max_model_len=4096,
            gpu_memory_utilization=0.6,
            enforce_eager=True,
            quantization="awq"
        )

        prompts = ["<|im_start|>user\nWho are you?<|im_end|>\n<|im_start|>assistant\n"]
        sampling_params = SamplingParams(temperature=0.7, top_p=0.9, max_tokens=100)
        
        print("\nStarting generation...")
        outputs = llm.generate(prompts, sampling_params)

        for output in outputs:
            print(f"\nResult: {output.outputs[0].text}\n")
    except Exception as e:
        import traceback
        traceback.print_exc()
        print(f"\nCRASH ENCOUNTERED: {e}")

if __name__ == "__main__":
    main()
