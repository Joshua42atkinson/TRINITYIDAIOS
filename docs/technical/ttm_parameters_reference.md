# TTM Parameter Reference for Strix Halo Unified Memory

## 📊 TTM Parameter Calculations

### **Formula**
```
TTM pages_limit = (Target_GB * 1024 * 1024 * 1024) / 4096
TTM page_pool_size = pages_limit (same value)
```

### **Common Configurations**

#### **96GB Unified Memory**
```
pages_limit = (96 * 1024 * 1024 * 1024) / 4096 = 25165824
page_pool_size = 25165824
```

#### **128GB Unified Memory**
```
pages_limit = (128 * 1024 * 1024 * 1024) / 4096 = 33554432
page_pool_size = 33554432
```

#### **64GB Unified Memory**
```
pages_limit = (64 * 1024 * 1024 * 1024) / 4096 = 16777216
page_pool_size = 16777216
```

## 🔧 GRUB Configuration

### **96GB Setup (Recommended for 97B Model)**
```bash
GRUB_CMDLINE_LINUX_DEFAULT="quiet splash amd_iommu=off ttm.pages_limit=25165824 ttm.page_pool_size=25165824"
```

### **128GB Setup (Maximum)**
```bash
GRUB_CMDLINE_LINUX_DEFAULT="quiet splash amd_iommu=off ttm.pages_limit=33554432 ttm.page_pool_size=33554432"
```

## ✅ Verification Commands

### **Check Memory Allocation**
```bash
cat /sys/module/ttm/parameters/p* | awk '{print $1 / (1024 * 1024 / 4)}'
# Should show: "96 96" for 96GB setup
```

### **Check ROCm Memory**
```bash
rocm-smi --showmeminfo vram
# Should show ~96GB total VRAM after reboot
```

## 🎯 Trinity Configuration

### **Current Setup**
- **BIOS GMA**: 512MB ✅
- **Target Memory**: 96GB (optimal for 97B model)
- **ROCm Version**: 7.2.0 ✅
- **Kernel**: 6.19.4 ✅

### **Next Steps**
1. Reboot to apply new TTM parameters
2. Verify memory allocation with verification script
3. Test 97B model loading
4. Monitor KV cache performance

## 🚀 Ready for 97B Model!

Once rebooted with these settings, Trinity will have:
- **96GB VRAM** for large language models
- **Optimal performance** for 97B model inference
- **Stable configuration** for production workloads
