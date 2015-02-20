use std::marker::PhantomData;

use Display;
use GlObject;
use buffer::Buffer;
use program;
use vertex::VertexFormat;

use libc;
use context;
use context::GlVersion;
use gl;

pub struct TransformFeedbackSession<'a> {
    buffer: gl::types::GLuint,
    program: gl::types::GLuint,
    marker1: PhantomData<&'a mut Buffer>,
    marker2: PhantomData<&'a Program>,
}

pub fn new_session<'a>(display: &Display, buffer: &'a mut Buffer, format: &VertexFormat,
                       program: &'a Program)
                       -> Option<TransformFeedbackSession<'a>>
{
    if display.context.context.get_version() < &GlVersion(3, 0) &&
        display.context.context.get_extensions().gl_ext_transform_feedback
    {
        return None;
    }

    if !is_transform_feedback_matching(format, program) {
        return None;    // FIXME: result type
    }

    Some(TransformFeedbackSession {
        buffer: buffer.get_id(),
        program: program.get_id(),
        marker1: PhantomData,
        marker2: PhantomData,
    })
}

pub fn is_transform_feedback_matching(format: &VertexFormat, program: &Program) -> bool {
    if program.get_transform_feedback_mode != Some(TransformFeedbackMode::Interleaved) {
        return false;       // TODO: 
    }

    let mut current_offset = 0;
    for var in program.get_transform_feedback_varyings() {
        if format.iter()
                 .find(|&(ref n, o, t)| n == var.name && o == current_offset && t == var.ty)
                 .is_none()
        {
            return false;
        }

        current_offset += var.size;
    }

    true
}
