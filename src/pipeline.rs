
use gfx;
use gfx::{
    VertexBuffer,
    ConstantBuffer,
    RenderTarget,
    DepthStencilTarget,
    DepthTarget,

    state::{
        Depth,
        Stencil,

        Comparison,

        StencilSide,
        StencilOp
    }
};

use vertex::Vertex;

fn depth_test() -> Depth {
    Depth {
        fun: Comparison::Less,
        write: true,
    }
}


fn stencil_test() -> Stencil {
    let side = StencilSide {
        fun: Comparison::Never,
        mask_read: 0,
        mask_write: 0,
        op_fail: StencilOp::Keep,
        op_depth_fail: StencilOp::Keep,
        op_pass: StencilOp::Keep,
    };

    Stencil {
        front: side,
        back: side,
    }
}


gfx_defines!(
    constant Transform {
        projection: [[f32; 4]; 4] = "projection",
        view: [[f32; 4]; 4] = "view",
    }

    pipeline pipe {
        vbuf: VertexBuffer<Vertex> = (),
        transform: ConstantBuffer<Transform> = "Transform",
        out_color: RenderTarget<::ColorFormat> = "color",
        out_depth_stencil: DepthTarget<::DepthFormat> = super::depth_test(),
    }
);