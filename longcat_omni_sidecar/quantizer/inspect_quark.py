import quark.torch.quantization.config.config as c
from quark.torch.quantization.api import ModelQuantizer

print("Testing old way...")
try:
    quant_schema = c.QuantizationConfig(weight=c.Int4PerGroupSpec(ch_axis=1, group_size=128))
    quant_config = c.QConfig(
        global_quant_config=quant_schema,
        algo_config=[c.AWQConfig()],
        exclude=["router", "classifier", "lm_head", "linear1", "linear2"]
    )
    quantizer = ModelQuantizer(quant_config)
    print("Old way succeeded?!")
except Exception as e:
    import traceback
    print("Old way failed:")
    traceback.print_exc()

print("\nTesting new way with QLayerConfig...")
try:
    quant_schema = c.QLayerConfig(weight=[c.Int4PerGroupSpec(ch_axis=1, group_size=128)])
    # We don't know the exact signature of QLayerConfig yet, but let's test.
    quant_config = c.QConfig(
        global_quant_config=quant_schema,
        algo_config=[c.AWQConfig()],
        exclude=["router", "classifier", "lm_head", "linear1", "linear2"]
    )
    quantizer = ModelQuantizer(quant_config)
    print("New way succeeded!")
except Exception as e:
    print("New way failed:", e)

