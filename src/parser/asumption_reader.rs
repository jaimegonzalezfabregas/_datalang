use super::{
    conditional_reader::Conditional,
    defered_relation_reader::{read_defered_relation, DeferedRelation},
    error::{FailureExplanation, ParserError}, update_reader::{Update, read_update}, inmediate_relation_reader::{read_inmediate_relation, InmediateRelation},
};
use crate::{
    lexer::{self},
    parser::conditional_reader::read_conditional,
};

#[derive(Debug, Clone)]
pub enum Asumption {
    RelationInmediate(InmediateRelation),
    RelationDefered(DeferedRelation),
    Conditional(Conditional),
    Update(Update)
}

pub fn read_asumption(
    lexograms: &Vec<lexer::Lexogram>,
    start_cursor: usize,
    debug_margin: String,
    debug_print: bool,
) -> Result<Result<(Asumption, usize), FailureExplanation>, ParserError> {
    let a;
    let b;
    let c;
    let d;
    match read_inmediate_relation(
        lexograms,
        start_cursor,
        debug_margin.clone() + "   ",
        debug_print,
    )? {
        Ok((i_rel, jump_to)) => return Ok(Ok((Asumption::RelationInmediate(i_rel), jump_to))),
        Err(e) => a = e,
    }
    match read_defered_relation(
        lexograms,
        start_cursor,
        false,
        debug_margin.clone() + "   ",
        debug_print,
    )? {
        Ok((d_rel, jump_to)) => return Ok(Ok((Asumption::RelationDefered(d_rel), jump_to))),
        Err(e) => b = e,
    }
    match read_conditional(
        lexograms,
        start_cursor,
        debug_margin.clone() + "   ",
        debug_print,
    )? {
        Ok((ret, jump_to)) => return Ok(Ok((Asumption::Conditional(ret), jump_to))),
        Err(e) => c = e,
    }
    match read_update(
        lexograms,
        start_cursor,
        debug_margin.clone() + "   ",
        debug_print,
    )? {
        Ok((ret, jump_to)) => return Ok(Ok((Asumption::Update(ret), jump_to))),
        Err(e) => d = e,
    }

    Ok(Err(FailureExplanation {
        lex_pos: start_cursor,
        if_it_was: "asumption".into(),
        failed_because: "wasnt neither an extensional nor an intensional statement".into(),
        parent_failure: vec![a, b],
    }))
}
