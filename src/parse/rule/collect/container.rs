/*
 * parse/rule/collect/container.rs
 *
 * ftml - Library to parse Wikidot text
 * Copyright (C) 2019-2020 Ammon Smith
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

//! Helper code to parse tokens out to generate recursive containers.

use super::prelude::*;
use crate::parse::rule::collect::try_consume;
use crate::tree::{Container, ContainerType, Element};

/// Generic function to consume tokens into a container.
///
/// This is a subset of the functionality provided by `try_collect`,
/// as it builds `Container`s specifically.
///
/// The arguments which differ from `collect_until` are listed:
/// See that function for full documentation, as the call here
/// mostly wraps it.
///
/// The kind of container we're building:
/// Must match the parse rule.
/// * `container_type`
pub fn try_container<'p, 'r, 't>(
    log: &slog::Logger,
    parser: &'p mut Parser<'r, 't>,
    rule: Rule,
    container_type: ContainerType,
    close_conditions: &[ParseCondition],
    invalid_conditions: &[ParseCondition],
) -> ParseResult<'r, 't, Element<'t>> {
    // Log try_container() call
    let log = &log.new(slog_o!(
        "container-type" => str!(container_type.name()),
    ));

    info!(
        log,
        "Trying to consume tokens to produce container for {:?}", rule,
    );

    // Iterate and consume all the tokens
    let (elements, _, exceptions) =
        try_consume(log, parser, rule, close_conditions, invalid_conditions)?.into();

    // Package into a container
    ok!(
        Element::Container(Container::new(container_type, elements)),
        exceptions,
    )
}
