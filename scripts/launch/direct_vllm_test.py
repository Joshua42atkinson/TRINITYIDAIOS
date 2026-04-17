# Test direct vLLM engine initialization inside distrobox
import sys
try:
    from vllm import LLM, SamplingParams
except ImportError as e:
    print(f"Failed to import vllm: {e}")
    sys.exit(1)

model_path = "/home/joshua/trinity-models/omni/LongCat-Next-INT4"

print("Booting vLLM Engine directly...")
try:
    llm = LLM(
        model=model_path,
        quantization="awq",
        trust_remote_code=True,
        tensor_parallel_size=1,
        max_model_len=8192,
        gpu_memory_utilization=0.55,
        enforce_eager=True,
    )
    
    prompts = ["You are Pete. Are you alive?"]
    sampling_params = SamplingParams(temperature=0.7, top_p=0.9, max_tokens=100)
    
    print("Engine booted. Generating...")
    outputs = llm.generate(prompts, sampling_params)
    for output in outputs:
        prompt = output.prompt
        generated_text = output.outputs[0].text
        print(f"Pete Responds: {generated_text}")
    print("SUCCESS")
except Exception as e:
    import traceback
    print(f"vLLM explicit crash: {e}")
    traceback.print_exc()
    sys.exit(1)
