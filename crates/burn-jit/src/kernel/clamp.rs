use cubecl::prelude::*;

use crate::kernel::{launch_unary, UnaryOp};
use crate::{element::JitElement, tensor::JitTensor, JitRuntime};

#[derive(CubeLaunch)]
struct Options<C: Numeric> {
    min_value: C,
    max_value: C,
}

pub(crate) fn clamp<R: JitRuntime, E: JitElement>(
    input: JitTensor<R>,
    min_value: E,
    max_value: E,
) -> JitTensor<R> {
    struct ClampOp;

    impl<C: Numeric> UnaryOp<C> for ClampOp {
        type Options = Options<C>;

        fn __expand_execute(
            context: &mut CubeContext,
            input: <Line<C> as CubeType>::ExpandType,
            options: OptionsExpand<C>,
        ) -> <Line<C> as CubeType>::ExpandType {
            #[cube]
            fn execute<C: Numeric>(input: Line<C>, options: &Options<C>) -> Line<C> {
                Line::clamp(
                    input,
                    Line::new(options.min_value),
                    Line::new(options.max_value),
                )
            }

            execute::expand(context, input, options)
        }
    }

    launch_unary::<R, E, ClampOp, _>(input, |_| {
        OptionsLaunch::new(ScalarArg::new(min_value), ScalarArg::new(max_value))
    })
}
