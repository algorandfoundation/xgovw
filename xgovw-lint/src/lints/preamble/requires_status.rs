/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use annotate_snippets::snippet::{Annotation, AnnotationType, Slice, Snippet, SourceAnnotation};

use crate::lints::{Context, Error, FetchContext, Lint};

use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug)]
pub struct RequiresStatus<'n> {
    pub requires: &'n str,
    pub status: &'n str,
    pub flow: &'n [&'n [&'n str]],
}

impl<'n> RequiresStatus<'n> {
    fn tier(&self, map: &HashMap<&str, usize>, ctx: &Context<'_, '_>) -> usize {
        ctx.preamble()
            .by_name(self.status)
            .map(|f| f.value())
            .map(str::trim)
            .and_then(|s| map.get(s))
            .copied()
            .unwrap_or(0)
    }
}

impl<'n> Lint for RequiresStatus<'n> {
    fn find_resources<'a>(&self, ctx: &FetchContext<'a>) -> Result<(), Error> {
        let field = match ctx.preamble().by_name(self.requires) {
            None => return Ok(()),
            Some(s) => s,
        };

        field
            .value()
            .split(',')
            .map(str::trim)
            .map(str::parse::<u64>)
            .filter_map(Result::ok)
            .map(|n| {
                let mut number = format!("xgov-{}.md", n);
                if n < 10 {
                    number = format!("xgov-{}{}.md", "000", n);
                } else if n < 100 {
                    number = format!("xgov-{}{}.md", "00", n);
                } else if n < 1000 {
                    number = format!("xgov-{}{}.md", "0", n);
                }
                number
            })
            .map(PathBuf::from)
            .for_each(|p| ctx.fetch(p));

        Ok(())
    }

    fn lint<'a, 'b>(&self, slug: &'a str, ctx: &Context<'a, 'b>) -> Result<(), Error> {
        let field = match ctx.preamble().by_name(self.requires) {
            None => return Ok(()),
            Some(s) => s,
        };

        let mut map = HashMap::new();
        for (tier, values) in self.flow.iter().enumerate() {
            for value in *values {
                map.insert(*value, tier + 1);
            }
        }

        let my_tier = self.tier(&map, ctx);
        let mut too_unstable = Vec::new();
        let mut min = usize::MAX;

        let items = field.value().split(',');

        let mut offset = 0;
        for item in items {
            let current = offset;
            offset += item.len() + 1;

            let key = match item.trim().parse::<u64>() {
                Ok(k) => {
                    let mut number = format!("xgov-{}.md", k);
                    if k < 10 {
                        number = format!("xgov-{}{}.md", "000", k);
                    } else if k < 100 {
                        number = format!("xgov-{}{}.md", "00", k);
                    } else if k < 1000 {
                        number = format!("xgov-{}{}.md", "0", k);
                    }
                    PathBuf::from(number)
                }
                _ => continue,
            };
            let xgov = match ctx.xgov(&key) {
                Ok(xgov) => xgov,
                Err(e) => {
                    let label = format!("unable to read file `{}`: {}", key.display(), e);
                    ctx.report(Snippet {
                        title: Some(Annotation {
                            id: Some(slug),
                            label: Some(&label),
                            annotation_type: AnnotationType::Error,
                        }),
                        slices: vec![Slice {
                            fold: false,
                            line_start: field.line_start(),
                            origin: ctx.origin(),
                            source: field.source(),
                            annotations: vec![SourceAnnotation {
                                annotation_type: AnnotationType::Error,
                                label: "required from here",
                                range: (
                                    field.name().len() + current + 1,
                                    field.name().len() + current + 1 + item.len(),
                                ),
                            }],
                        }],
                        ..Default::default()
                    })?;
                    continue;
                }
            };
            let their_tier = self.tier(&map, &xgov);

            if their_tier < min {
                min = their_tier;
            }

            if their_tier >= my_tier {
                continue;
            }

            too_unstable.push(SourceAnnotation {
                annotation_type: AnnotationType::Error,
                label: "has a less advanced status",
                range: (
                    field.name().len() + current + 1,
                    field.name().len() + current + 1 + item.len(),
                ),
            });
        }

        if !too_unstable.is_empty() {
            let label = format!(
                "preamble header `{}` contains items not stable enough for a `{}` of `{}`",
                self.requires,
                self.status,
                ctx.preamble()
                    .by_name(self.status)
                    .map(|f| f.value())
                    .unwrap_or("<missing>")
                    .trim(),
            );

            let mut choices = map
                .iter()
                .filter_map(|(v, t)| if *t <= min { Some(v) } else { None })
                .map(ToString::to_string)
                .collect::<Vec<_>>();
            choices.sort();

            let choices = choices.join("`, `");

            let mut footer = vec![];
            let footer_label = format!(
                "valid `{}` values for this proposal are: `{}`",
                self.status, choices
            );

            if !choices.is_empty() {
                footer.push(Annotation {
                    annotation_type: AnnotationType::Help,
                    id: None,
                    label: Some(&footer_label),
                });
            }

            ctx.report(Snippet {
                title: Some(Annotation {
                    annotation_type: AnnotationType::Error,
                    id: Some(slug),
                    label: Some(&label),
                }),
                slices: vec![Slice {
                    fold: false,
                    line_start: field.line_start(),
                    origin: ctx.origin(),
                    source: field.source(),
                    annotations: too_unstable,
                }],
                footer,
                opt: Default::default(),
            })?;
        }

        Ok(())
    }
}
