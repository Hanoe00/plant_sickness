// Generated from ONNX "C:\\Users\\domin\\plant_sickness\\model/plant_disease_model.onnx" by burn-onnx
use burn::prelude::*;
use burn::nn::Linear;
use burn::nn::LinearConfig;
use burn::nn::LinearLayout;
use burn::nn::PaddingConfig2d;
use burn::nn::conv::Conv2d;
use burn::nn::conv::Conv2dConfig;
use burn::tensor::Bytes;
use burn_store::BurnpackStore;
use burn_store::ModuleSnapshot;


#[derive(Module, Debug)]
pub struct Model<B: Backend> {
    conv2d1: Conv2d<B>,
    conv2d2: Conv2d<B>,
    conv2d3: Conv2d<B>,
    conv2d4: Conv2d<B>,
    conv2d5: Conv2d<B>,
    conv2d6: Conv2d<B>,
    conv2d7: Conv2d<B>,
    conv2d8: Conv2d<B>,
    conv2d9: Conv2d<B>,
    conv2d10: Conv2d<B>,
    conv2d11: Conv2d<B>,
    conv2d12: Conv2d<B>,
    conv2d13: Conv2d<B>,
    conv2d14: Conv2d<B>,
    conv2d15: Conv2d<B>,
    conv2d16: Conv2d<B>,
    conv2d17: Conv2d<B>,
    conv2d18: Conv2d<B>,
    conv2d19: Conv2d<B>,
    conv2d20: Conv2d<B>,
    conv2d21: Conv2d<B>,
    conv2d22: Conv2d<B>,
    conv2d23: Conv2d<B>,
    conv2d24: Conv2d<B>,
    conv2d25: Conv2d<B>,
    conv2d26: Conv2d<B>,
    conv2d27: Conv2d<B>,
    conv2d28: Conv2d<B>,
    conv2d29: Conv2d<B>,
    conv2d30: Conv2d<B>,
    conv2d31: Conv2d<B>,
    conv2d32: Conv2d<B>,
    conv2d33: Conv2d<B>,
    conv2d34: Conv2d<B>,
    conv2d35: Conv2d<B>,
    conv2d36: Conv2d<B>,
    conv2d37: Conv2d<B>,
    conv2d38: Conv2d<B>,
    conv2d39: Conv2d<B>,
    conv2d40: Conv2d<B>,
    conv2d41: Conv2d<B>,
    conv2d42: Conv2d<B>,
    conv2d43: Conv2d<B>,
    conv2d44: Conv2d<B>,
    conv2d45: Conv2d<B>,
    conv2d46: Conv2d<B>,
    conv2d47: Conv2d<B>,
    conv2d48: Conv2d<B>,
    conv2d49: Conv2d<B>,
    conv2d50: Conv2d<B>,
    conv2d51: Conv2d<B>,
    conv2d52: Conv2d<B>,
    conv2d53: Conv2d<B>,
    conv2d54: Conv2d<B>,
    conv2d55: Conv2d<B>,
    conv2d56: Conv2d<B>,
    conv2d57: Conv2d<B>,
    conv2d58: Conv2d<B>,
    conv2d59: Conv2d<B>,
    conv2d60: Conv2d<B>,
    conv2d61: Conv2d<B>,
    conv2d62: Conv2d<B>,
    linear1: Linear<B>,
    linear2: Linear<B>,
    phantom: core::marker::PhantomData<B>,
    #[module(skip)]
    device: B::Device,
}


extern crate std;

impl<B: Backend> Default for Model<B> {
    fn default() -> Self {
        Self::from_file(
            "C:\\Users\\domin\\plant_sickness\\src\\plant_disease_model.bpk",
            &Default::default(),
        )
    }
}

impl<B: Backend> Model<B> {
    /// Load model weights from a burnpack file.
    pub fn from_file<P: AsRef<std::path::Path>>(file: P, device: &B::Device) -> Self {
        let mut model = Self::new(device);
        let mut store = BurnpackStore::from_file(file);
        model.load_from(&mut store).expect("Failed to load burnpack file");
        model
    }

    /// Load model weights from in-memory bytes.
    ///
    /// The bytes must be the contents of a `.bpk` file.
    pub fn from_bytes(bytes: Bytes, device: &B::Device) -> Self {
        let mut model = Self::new(device);
        let mut store = BurnpackStore::from_bytes(Some(bytes));
        model.load_from(&mut store).expect("Failed to load burnpack bytes");
        model
    }
}

impl<B: Backend> Model<B> {
    #[allow(unused_variables)]
    pub fn new(device: &B::Device) -> Self {
        let conv2d1 = Conv2dConfig::new([3, 16], [3, 3])
            .with_stride([2, 2])
            .with_padding(PaddingConfig2d::Explicit(1, 1, 1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d2 = Conv2dConfig::new([16, 16], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1, 1, 1))
            .with_dilation([1, 1])
            .with_groups(16)
            .with_bias(true)
            .init(device);
        let conv2d3 = Conv2dConfig::new([16, 16], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d4 = Conv2dConfig::new([16, 64], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d5 = Conv2dConfig::new([64, 64], [3, 3])
            .with_stride([2, 2])
            .with_padding(PaddingConfig2d::Explicit(1, 1, 1, 1))
            .with_dilation([1, 1])
            .with_groups(64)
            .with_bias(true)
            .init(device);
        let conv2d6 = Conv2dConfig::new([64, 24], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d7 = Conv2dConfig::new([24, 72], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d8 = Conv2dConfig::new([72, 72], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1, 1, 1))
            .with_dilation([1, 1])
            .with_groups(72)
            .with_bias(true)
            .init(device);
        let conv2d9 = Conv2dConfig::new([72, 24], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d10 = Conv2dConfig::new([24, 72], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d11 = Conv2dConfig::new([72, 72], [5, 5])
            .with_stride([2, 2])
            .with_padding(PaddingConfig2d::Explicit(2, 2, 2, 2))
            .with_dilation([1, 1])
            .with_groups(72)
            .with_bias(true)
            .init(device);
        let conv2d12 = Conv2dConfig::new([72, 24], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d13 = Conv2dConfig::new([24, 72], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d14 = Conv2dConfig::new([72, 40], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d15 = Conv2dConfig::new([40, 120], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d16 = Conv2dConfig::new([120, 120], [5, 5])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(2, 2, 2, 2))
            .with_dilation([1, 1])
            .with_groups(120)
            .with_bias(true)
            .init(device);
        let conv2d17 = Conv2dConfig::new([120, 32], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d18 = Conv2dConfig::new([32, 120], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d19 = Conv2dConfig::new([120, 40], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d20 = Conv2dConfig::new([40, 120], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d21 = Conv2dConfig::new([120, 120], [5, 5])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(2, 2, 2, 2))
            .with_dilation([1, 1])
            .with_groups(120)
            .with_bias(true)
            .init(device);
        let conv2d22 = Conv2dConfig::new([120, 32], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d23 = Conv2dConfig::new([32, 120], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d24 = Conv2dConfig::new([120, 40], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d25 = Conv2dConfig::new([40, 240], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d26 = Conv2dConfig::new([240, 240], [3, 3])
            .with_stride([2, 2])
            .with_padding(PaddingConfig2d::Explicit(1, 1, 1, 1))
            .with_dilation([1, 1])
            .with_groups(240)
            .with_bias(true)
            .init(device);
        let conv2d27 = Conv2dConfig::new([240, 80], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d28 = Conv2dConfig::new([80, 200], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d29 = Conv2dConfig::new([200, 200], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1, 1, 1))
            .with_dilation([1, 1])
            .with_groups(200)
            .with_bias(true)
            .init(device);
        let conv2d30 = Conv2dConfig::new([200, 80], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d31 = Conv2dConfig::new([80, 184], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d32 = Conv2dConfig::new([184, 184], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1, 1, 1))
            .with_dilation([1, 1])
            .with_groups(184)
            .with_bias(true)
            .init(device);
        let conv2d33 = Conv2dConfig::new([184, 80], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d34 = Conv2dConfig::new([80, 184], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d35 = Conv2dConfig::new([184, 184], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1, 1, 1))
            .with_dilation([1, 1])
            .with_groups(184)
            .with_bias(true)
            .init(device);
        let conv2d36 = Conv2dConfig::new([184, 80], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d37 = Conv2dConfig::new([80, 480], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d38 = Conv2dConfig::new([480, 480], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1, 1, 1))
            .with_dilation([1, 1])
            .with_groups(480)
            .with_bias(true)
            .init(device);
        let conv2d39 = Conv2dConfig::new([480, 120], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d40 = Conv2dConfig::new([120, 480], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d41 = Conv2dConfig::new([480, 112], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d42 = Conv2dConfig::new([112, 672], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d43 = Conv2dConfig::new([672, 672], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1, 1, 1))
            .with_dilation([1, 1])
            .with_groups(672)
            .with_bias(true)
            .init(device);
        let conv2d44 = Conv2dConfig::new([672, 168], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d45 = Conv2dConfig::new([168, 672], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d46 = Conv2dConfig::new([672, 112], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d47 = Conv2dConfig::new([112, 672], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d48 = Conv2dConfig::new([672, 672], [5, 5])
            .with_stride([2, 2])
            .with_padding(PaddingConfig2d::Explicit(2, 2, 2, 2))
            .with_dilation([1, 1])
            .with_groups(672)
            .with_bias(true)
            .init(device);
        let conv2d49 = Conv2dConfig::new([672, 168], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d50 = Conv2dConfig::new([168, 672], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d51 = Conv2dConfig::new([672, 160], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d52 = Conv2dConfig::new([160, 960], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d53 = Conv2dConfig::new([960, 960], [5, 5])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(2, 2, 2, 2))
            .with_dilation([1, 1])
            .with_groups(960)
            .with_bias(true)
            .init(device);
        let conv2d54 = Conv2dConfig::new([960, 240], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d55 = Conv2dConfig::new([240, 960], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d56 = Conv2dConfig::new([960, 160], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d57 = Conv2dConfig::new([160, 960], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d58 = Conv2dConfig::new([960, 960], [5, 5])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(2, 2, 2, 2))
            .with_dilation([1, 1])
            .with_groups(960)
            .with_bias(true)
            .init(device);
        let conv2d59 = Conv2dConfig::new([960, 240], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d60 = Conv2dConfig::new([240, 960], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d61 = Conv2dConfig::new([960, 160], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d62 = Conv2dConfig::new([160, 960], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let linear1 = LinearConfig::new(960, 1280)
            .with_bias(true)
            .with_layout(LinearLayout::Col)
            .init(device);
        let linear2 = LinearConfig::new(1280, 38)
            .with_bias(true)
            .with_layout(LinearLayout::Col)
            .init(device);
        Self {
            conv2d1,
            conv2d2,
            conv2d3,
            conv2d4,
            conv2d5,
            conv2d6,
            conv2d7,
            conv2d8,
            conv2d9,
            conv2d10,
            conv2d11,
            conv2d12,
            conv2d13,
            conv2d14,
            conv2d15,
            conv2d16,
            conv2d17,
            conv2d18,
            conv2d19,
            conv2d20,
            conv2d21,
            conv2d22,
            conv2d23,
            conv2d24,
            conv2d25,
            conv2d26,
            conv2d27,
            conv2d28,
            conv2d29,
            conv2d30,
            conv2d31,
            conv2d32,
            conv2d33,
            conv2d34,
            conv2d35,
            conv2d36,
            conv2d37,
            conv2d38,
            conv2d39,
            conv2d40,
            conv2d41,
            conv2d42,
            conv2d43,
            conv2d44,
            conv2d45,
            conv2d46,
            conv2d47,
            conv2d48,
            conv2d49,
            conv2d50,
            conv2d51,
            conv2d52,
            conv2d53,
            conv2d54,
            conv2d55,
            conv2d56,
            conv2d57,
            conv2d58,
            conv2d59,
            conv2d60,
            conv2d61,
            conv2d62,
            linear1,
            linear2,
            phantom: core::marker::PhantomData,
            device: device.clone(),
        }
    }

    #[allow(clippy::let_and_return, clippy::approx_constant)]
    pub fn forward(&self, input: Tensor<B, 4>) -> Tensor<B, 2> {
        let constant130_out1: [i64; 1] = [960i64];
        let shape1_out1: [i64; 1] = {
            let axes = &input.clone().dims()[0..1];
            let mut output = [0i64; 1];
            for i in 0..1 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let conv2d1_out1 = self.conv2d1.forward(input);
        let hardswish1_out1 = burn::tensor::activation::hard_swish(conv2d1_out1);
        let conv2d2_out1 = self.conv2d2.forward(hardswish1_out1.clone());
        let relu1_out1 = burn::tensor::activation::relu(conv2d2_out1);
        let conv2d3_out1 = self.conv2d3.forward(relu1_out1);
        let add1_out1 = conv2d3_out1.add(hardswish1_out1);
        let conv2d4_out1 = self.conv2d4.forward(add1_out1);
        let relu2_out1 = burn::tensor::activation::relu(conv2d4_out1);
        let conv2d5_out1 = self.conv2d5.forward(relu2_out1);
        let relu3_out1 = burn::tensor::activation::relu(conv2d5_out1);
        let conv2d6_out1 = self.conv2d6.forward(relu3_out1);
        let conv2d7_out1 = self.conv2d7.forward(conv2d6_out1.clone());
        let relu4_out1 = burn::tensor::activation::relu(conv2d7_out1);
        let conv2d8_out1 = self.conv2d8.forward(relu4_out1);
        let relu5_out1 = burn::tensor::activation::relu(conv2d8_out1);
        let conv2d9_out1 = self.conv2d9.forward(relu5_out1);
        let add2_out1 = conv2d9_out1.add(conv2d6_out1);
        let conv2d10_out1 = self.conv2d10.forward(add2_out1);
        let relu6_out1 = burn::tensor::activation::relu(conv2d10_out1);
        let conv2d11_out1 = self.conv2d11.forward(relu6_out1);
        let relu7_out1 = burn::tensor::activation::relu(conv2d11_out1);
        let reducemean1_out1 = { relu7_out1.clone().mean_dim(2usize).mean_dim(3usize) };
        let conv2d12_out1 = self.conv2d12.forward(reducemean1_out1);
        let relu8_out1 = burn::tensor::activation::relu(conv2d12_out1);
        let conv2d13_out1 = self.conv2d13.forward(relu8_out1);
        let hardsigmoid1_out1 = burn::tensor::activation::hard_sigmoid(
            conv2d13_out1,
            0.1666666716337204,
            0.5,
        );
        let mul1_out1 = hardsigmoid1_out1.mul(relu7_out1);
        let conv2d14_out1 = self.conv2d14.forward(mul1_out1);
        let conv2d15_out1 = self.conv2d15.forward(conv2d14_out1.clone());
        let relu9_out1 = burn::tensor::activation::relu(conv2d15_out1);
        let conv2d16_out1 = self.conv2d16.forward(relu9_out1);
        let relu10_out1 = burn::tensor::activation::relu(conv2d16_out1);
        let reducemean2_out1 = { relu10_out1.clone().mean_dim(2usize).mean_dim(3usize) };
        let conv2d17_out1 = self.conv2d17.forward(reducemean2_out1);
        let relu11_out1 = burn::tensor::activation::relu(conv2d17_out1);
        let conv2d18_out1 = self.conv2d18.forward(relu11_out1);
        let hardsigmoid2_out1 = burn::tensor::activation::hard_sigmoid(
            conv2d18_out1,
            0.1666666716337204,
            0.5,
        );
        let mul2_out1 = hardsigmoid2_out1.mul(relu10_out1);
        let conv2d19_out1 = self.conv2d19.forward(mul2_out1);
        let add3_out1 = conv2d19_out1.add(conv2d14_out1);
        let conv2d20_out1 = self.conv2d20.forward(add3_out1.clone());
        let relu12_out1 = burn::tensor::activation::relu(conv2d20_out1);
        let conv2d21_out1 = self.conv2d21.forward(relu12_out1);
        let relu13_out1 = burn::tensor::activation::relu(conv2d21_out1);
        let reducemean3_out1 = { relu13_out1.clone().mean_dim(2usize).mean_dim(3usize) };
        let conv2d22_out1 = self.conv2d22.forward(reducemean3_out1);
        let relu14_out1 = burn::tensor::activation::relu(conv2d22_out1);
        let conv2d23_out1 = self.conv2d23.forward(relu14_out1);
        let hardsigmoid3_out1 = burn::tensor::activation::hard_sigmoid(
            conv2d23_out1,
            0.1666666716337204,
            0.5,
        );
        let mul3_out1 = hardsigmoid3_out1.mul(relu13_out1);
        let conv2d24_out1 = self.conv2d24.forward(mul3_out1);
        let add4_out1 = conv2d24_out1.add(add3_out1);
        let conv2d25_out1 = self.conv2d25.forward(add4_out1);
        let hardswish2_out1 = burn::tensor::activation::hard_swish(conv2d25_out1);
        let conv2d26_out1 = self.conv2d26.forward(hardswish2_out1);
        let hardswish3_out1 = burn::tensor::activation::hard_swish(conv2d26_out1);
        let conv2d27_out1 = self.conv2d27.forward(hardswish3_out1);
        let conv2d28_out1 = self.conv2d28.forward(conv2d27_out1.clone());
        let hardswish4_out1 = burn::tensor::activation::hard_swish(conv2d28_out1);
        let conv2d29_out1 = self.conv2d29.forward(hardswish4_out1);
        let hardswish5_out1 = burn::tensor::activation::hard_swish(conv2d29_out1);
        let conv2d30_out1 = self.conv2d30.forward(hardswish5_out1);
        let add5_out1 = conv2d30_out1.add(conv2d27_out1);
        let conv2d31_out1 = self.conv2d31.forward(add5_out1.clone());
        let hardswish6_out1 = burn::tensor::activation::hard_swish(conv2d31_out1);
        let conv2d32_out1 = self.conv2d32.forward(hardswish6_out1);
        let hardswish7_out1 = burn::tensor::activation::hard_swish(conv2d32_out1);
        let conv2d33_out1 = self.conv2d33.forward(hardswish7_out1);
        let add6_out1 = conv2d33_out1.add(add5_out1);
        let conv2d34_out1 = self.conv2d34.forward(add6_out1.clone());
        let hardswish8_out1 = burn::tensor::activation::hard_swish(conv2d34_out1);
        let conv2d35_out1 = self.conv2d35.forward(hardswish8_out1);
        let hardswish9_out1 = burn::tensor::activation::hard_swish(conv2d35_out1);
        let conv2d36_out1 = self.conv2d36.forward(hardswish9_out1);
        let add7_out1 = conv2d36_out1.add(add6_out1);
        let conv2d37_out1 = self.conv2d37.forward(add7_out1);
        let hardswish10_out1 = burn::tensor::activation::hard_swish(conv2d37_out1);
        let conv2d38_out1 = self.conv2d38.forward(hardswish10_out1);
        let hardswish11_out1 = burn::tensor::activation::hard_swish(conv2d38_out1);
        let reducemean4_out1 = {
            hardswish11_out1.clone().mean_dim(2usize).mean_dim(3usize)
        };
        let conv2d39_out1 = self.conv2d39.forward(reducemean4_out1);
        let relu15_out1 = burn::tensor::activation::relu(conv2d39_out1);
        let conv2d40_out1 = self.conv2d40.forward(relu15_out1);
        let hardsigmoid4_out1 = burn::tensor::activation::hard_sigmoid(
            conv2d40_out1,
            0.1666666716337204,
            0.5,
        );
        let mul4_out1 = hardsigmoid4_out1.mul(hardswish11_out1);
        let conv2d41_out1 = self.conv2d41.forward(mul4_out1);
        let conv2d42_out1 = self.conv2d42.forward(conv2d41_out1.clone());
        let hardswish12_out1 = burn::tensor::activation::hard_swish(conv2d42_out1);
        let conv2d43_out1 = self.conv2d43.forward(hardswish12_out1);
        let hardswish13_out1 = burn::tensor::activation::hard_swish(conv2d43_out1);
        let reducemean5_out1 = {
            hardswish13_out1.clone().mean_dim(2usize).mean_dim(3usize)
        };
        let conv2d44_out1 = self.conv2d44.forward(reducemean5_out1);
        let relu16_out1 = burn::tensor::activation::relu(conv2d44_out1);
        let conv2d45_out1 = self.conv2d45.forward(relu16_out1);
        let hardsigmoid5_out1 = burn::tensor::activation::hard_sigmoid(
            conv2d45_out1,
            0.1666666716337204,
            0.5,
        );
        let mul5_out1 = hardsigmoid5_out1.mul(hardswish13_out1);
        let conv2d46_out1 = self.conv2d46.forward(mul5_out1);
        let add8_out1 = conv2d46_out1.add(conv2d41_out1);
        let conv2d47_out1 = self.conv2d47.forward(add8_out1);
        let hardswish14_out1 = burn::tensor::activation::hard_swish(conv2d47_out1);
        let conv2d48_out1 = self.conv2d48.forward(hardswish14_out1);
        let hardswish15_out1 = burn::tensor::activation::hard_swish(conv2d48_out1);
        let reducemean6_out1 = {
            hardswish15_out1.clone().mean_dim(2usize).mean_dim(3usize)
        };
        let conv2d49_out1 = self.conv2d49.forward(reducemean6_out1);
        let relu17_out1 = burn::tensor::activation::relu(conv2d49_out1);
        let conv2d50_out1 = self.conv2d50.forward(relu17_out1);
        let hardsigmoid6_out1 = burn::tensor::activation::hard_sigmoid(
            conv2d50_out1,
            0.1666666716337204,
            0.5,
        );
        let mul6_out1 = hardsigmoid6_out1.mul(hardswish15_out1);
        let conv2d51_out1 = self.conv2d51.forward(mul6_out1);
        let conv2d52_out1 = self.conv2d52.forward(conv2d51_out1.clone());
        let hardswish16_out1 = burn::tensor::activation::hard_swish(conv2d52_out1);
        let conv2d53_out1 = self.conv2d53.forward(hardswish16_out1);
        let hardswish17_out1 = burn::tensor::activation::hard_swish(conv2d53_out1);
        let reducemean7_out1 = {
            hardswish17_out1.clone().mean_dim(2usize).mean_dim(3usize)
        };
        let conv2d54_out1 = self.conv2d54.forward(reducemean7_out1);
        let relu18_out1 = burn::tensor::activation::relu(conv2d54_out1);
        let conv2d55_out1 = self.conv2d55.forward(relu18_out1);
        let hardsigmoid7_out1 = burn::tensor::activation::hard_sigmoid(
            conv2d55_out1,
            0.1666666716337204,
            0.5,
        );
        let mul7_out1 = hardsigmoid7_out1.mul(hardswish17_out1);
        let conv2d56_out1 = self.conv2d56.forward(mul7_out1);
        let add9_out1 = conv2d56_out1.add(conv2d51_out1);
        let conv2d57_out1 = self.conv2d57.forward(add9_out1.clone());
        let hardswish18_out1 = burn::tensor::activation::hard_swish(conv2d57_out1);
        let conv2d58_out1 = self.conv2d58.forward(hardswish18_out1);
        let hardswish19_out1 = burn::tensor::activation::hard_swish(conv2d58_out1);
        let reducemean8_out1 = {
            hardswish19_out1.clone().mean_dim(2usize).mean_dim(3usize)
        };
        let conv2d59_out1 = self.conv2d59.forward(reducemean8_out1);
        let relu19_out1 = burn::tensor::activation::relu(conv2d59_out1);
        let conv2d60_out1 = self.conv2d60.forward(relu19_out1);
        let hardsigmoid8_out1 = burn::tensor::activation::hard_sigmoid(
            conv2d60_out1,
            0.1666666716337204,
            0.5,
        );
        let mul8_out1 = hardsigmoid8_out1.mul(hardswish19_out1);
        let conv2d61_out1 = self.conv2d61.forward(mul8_out1);
        let add10_out1 = conv2d61_out1.add(add9_out1);
        let conv2d62_out1 = self.conv2d62.forward(add10_out1);
        let hardswish20_out1 = burn::tensor::activation::hard_swish(conv2d62_out1);
        let reducemean9_out1 = { hardswish20_out1.mean_dim(2usize).mean_dim(3usize) };
        let concat1_out1: [i64; 2usize] = [&shape1_out1[..], &constant130_out1[..]]
            .concat()
            .try_into()
            .unwrap();
        let reshape1_out1 = reducemean9_out1.reshape(concat1_out1);
        let linear1_out1 = self.linear1.forward(reshape1_out1);
        let hardswish21_out1 = burn::tensor::activation::hard_swish(linear1_out1);
        let linear2_out1 = self.linear2.forward(hardswish21_out1);
        linear2_out1
    }
}
