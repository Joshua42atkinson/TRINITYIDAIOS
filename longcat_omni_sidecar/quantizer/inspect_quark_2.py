import inspect
import quark.torch.quantization.config.config as c

print("Available in config:", dir(c))

print("\n--- Int4PerGroupSpec ---")
print(inspect.signature(c.Int4PerGroupSpec.__init__))
print(dir(c.Int4PerGroupSpec))

# Find any TensorConfig or Spec classes
for name in dir(c):
    if 'Spec' in name or 'Config' in name or 'Quant' in name or 'Tensor' in name:
        obj = getattr(c, name)
        if hasattr(obj, '__init__'):
            try:
                sig = inspect.signature(obj.__init__)
                print(f"{name}: {sig}")
            except ValueError:
                pass
