import quark.torch.quantization.config.config as c
import inspect

print(inspect.getsource(c.Int4PerGroupSpec))
print(inspect.getsource(c.Bfp4PerGroupSpec))
print(inspect.getsource(c.QTensorConfig))
