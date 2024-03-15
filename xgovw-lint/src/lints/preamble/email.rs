/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use annotate_snippets::snippet::{Annotation, AnnotationType, Slice, Snippet, SourceAnnotation};
use regex::RegexSet;

use crate::lints::{Context, Error, Lint};

fn footer() -> Vec<Annotation<'static>> {
    vec![
        Annotation {
            annotation_type: AnnotationType::Help,
            id: None,
            label: Some("test@example.com"),
        }
    ]
}

#[derive(Debug)]
pub struct Email<'n>(pub &'n str);

impl<'n> Lint for Email<'n> {
    fn lint<'a, 'b>(&self, slug: &'a str, ctx: &Context<'a, 'b>) -> Result<(), Error> {
        let field = match ctx.preamble().by_name(self.0) {
            None => return Ok(()),
            Some(s) => s,
        };

        // TODO: Email addresses are insane, and can probably contain commas,
        //       parentheses, and greater-/less- than symbols. For correctness,
        //       we should switch to a parser that can handle those cases.

        let item = field.value();

        let set = RegexSet::new(&[
            r"^[^@][^>]*@[^>]+\.[^>]+$", // Match an email address.
        ])
        .unwrap();

        let offset = 0;


        let current = offset;
        let trimmed = item.trim();

        let matches = set.matches(trimmed);
        if matches.matched_any() == false {
            ctx.report(Snippet {
                title: Some(Annotation {
                    annotation_type: AnnotationType::Error,
                    id: Some(slug),
                    label: Some("email must match the expected format"),
                }),
                slices: vec![Slice {
                    fold: false,
                    line_start: field.line_start(),
                    origin: ctx.origin(),
                    source: field.source(),
                    annotations: vec![SourceAnnotation {
                        annotation_type: AnnotationType::Error,
                        label: "unrecognized email",
                        range: (
                            field.name().len() + current + 1,
                            field.name().len() + current + 1 + item.len(),
                        ),
                    }],
                }],
                footer: footer(),
                opt: Default::default(),
            })?;
        }
        
        


        Ok(())
    }
}
