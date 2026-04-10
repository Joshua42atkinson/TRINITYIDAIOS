import quark.torch.quantization.config.config as c
from quark.torch.quantization.api import ModelQuantizer

try:
    quant_schema = c.QLayerConfig(weight=[c.Int4PerGroupSpec(ch_axis=1, group_size=128).to_quantization_spec()])
    quant_config = c.QConfig(
        global_quant_config=quant_schema,
        algo_config=[c.AWQConfig()],
        exclude=["router", "classifier", "lm_head", "linear1", "linear2"]
    )
    print("Schema created successfully!")

    # Let's see if ModelQuantizer accepts it
    quantizer = ModelQuantizer(quant_config)
    print("ModelQuantizer created successfully with .to_quantization_spec()!")
except Exception as e:
    import traceback
    traceback.print_exc()
