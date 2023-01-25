use iced::{
    widget::{Column, Text},
    Renderer,
};

use crate::TabContext;
#[derive(Debug, Clone)]
pub enum Diff {
    Text {
        audio_item_key: String,
        before: String,
        after: String,
    },
    Pitch {
        audio_item_key: String,
        before: f64,
        after: f64,
    },
    Speed {
        audio_item_key: String,
        before: f64,
        after: f64,
    },
    Intonation {
        audio_item_key: String,
        before: f64,
        after: f64,
    },
    Volume {
        audio_item_key: String,
        before: f64,
        after: f64,
    },
    PrePhonemeLength {
        audio_item_key: String,
        before: f64,
        after: f64,
    },
    PostPhonemeLength {
        audio_item_key: String,
        before: f64,
        after: f64,
    },
}

#[derive(Debug, Clone)]
pub struct History {
    undo_stack: Vec<Diff>,
    redo_stack: Vec<Diff>,
    unsquashed_buffer: Vec<Diff>,
    depth: usize,
}
impl History {
    pub(crate) fn new() -> Self {
        Self {
            unsquashed_buffer: Vec::new(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            depth: 0,
        }
    }
    /// undo
    pub(crate) fn undo(&mut self, tab_context: &mut TabContext) {
        if let Some(diff) = self.undo_stack.pop() {
            self.redo_stack.push(diff.clone());
            match diff {
                Diff::Text {
                    audio_item_key,
                    before,
                    after: _,
                } => {
                    if let Some(ai) = tab_context.project.audioItems.get_mut(&audio_item_key) {
                        ai.text = before;
                    }
                }
                Diff::Pitch {
                    audio_item_key,
                    before,
                    after: _,
                } => {
                    if let Some(ai) = tab_context.project.audioItems.get_mut(&audio_item_key) {
                        if let Some(query) = &mut ai.query {
                            query.pitchScale = before;
                        }
                    }
                }
                Diff::Speed {
                    audio_item_key,
                    before,
                    after: _,
                } => {
                    if let Some(ai) = tab_context.project.audioItems.get_mut(&audio_item_key) {
                        if let Some(query) = &mut ai.query {
                            query.speedScale = before;
                        }
                    }
                }
                Diff::Intonation {
                    audio_item_key,
                    before,
                    after: _,
                } => {
                    if let Some(ai) = tab_context.project.audioItems.get_mut(&audio_item_key) {
                        if let Some(query) = &mut ai.query {
                            query.intonationScale = before;
                        }
                    }
                }
                Diff::Volume {
                    audio_item_key,
                    before,
                    after: _,
                } => {
                    if let Some(ai) = tab_context.project.audioItems.get_mut(&audio_item_key) {
                        if let Some(query) = &mut ai.query {
                            query.volumeScale = before;
                        }
                    }
                }
                Diff::PrePhonemeLength {
                    audio_item_key,
                    before,
                    after: _,
                } => {
                    if let Some(ai) = tab_context.project.audioItems.get_mut(&audio_item_key) {
                        if let Some(query) = &mut ai.query {
                            query.prePhonemeLength = before;
                        }
                    }
                }
                Diff::PostPhonemeLength {
                    audio_item_key,
                    before,
                    after: _,
                } => {
                    if let Some(ai) = tab_context.project.audioItems.get_mut(&audio_item_key) {
                        if let Some(query) = &mut ai.query {
                            query.postPhonemeLength = before;
                        }
                    }
                }
            }

            self.depth += 1;
        }
    }
    /// redo
    pub(crate) fn redo(&mut self, tab_context: &mut TabContext) {
        if let Some(diff) = self.redo_stack.pop() {
            self.undo_stack.push(diff.clone());
            match diff {
                Diff::Text {
                    audio_item_key,
                    before: _,
                    after,
                } => {
                    if let Some(ai) = tab_context.project.audioItems.get_mut(&audio_item_key) {
                        ai.text = after;
                    }
                }
                Diff::Pitch {
                    audio_item_key,
                    before: _,
                    after,
                } => {
                    if let Some(ai) = tab_context.project.audioItems.get_mut(&audio_item_key) {
                        if let Some(query) = &mut ai.query {
                            query.pitchScale = after;
                        }
                    }
                }
                Diff::Speed {
                    audio_item_key,
                    before: _,
                    after,
                } => {
                    if let Some(ai) = tab_context.project.audioItems.get_mut(&audio_item_key) {
                        if let Some(query) = &mut ai.query {
                            query.speedScale = after;
                        }
                    }
                }
                Diff::Intonation {
                    audio_item_key,
                    before: _,
                    after,
                } => {
                    if let Some(ai) = tab_context.project.audioItems.get_mut(&audio_item_key) {
                        if let Some(query) = &mut ai.query {
                            query.intonationScale = after;
                        }
                    }
                }
                Diff::Volume {
                    audio_item_key,
                    before: _,
                    after,
                } => {
                    if let Some(ai) = tab_context.project.audioItems.get_mut(&audio_item_key) {
                        if let Some(query) = &mut ai.query {
                            query.volumeScale = after;
                        }
                    }
                }
                Diff::PrePhonemeLength {
                    audio_item_key,
                    before: _,
                    after,
                } => {
                    if let Some(ai) = tab_context.project.audioItems.get_mut(&audio_item_key) {
                        if let Some(query) = &mut ai.query {
                            query.prePhonemeLength = after;
                        }
                    }
                }
                Diff::PostPhonemeLength {
                    audio_item_key,
                    before: _,
                    after,
                } => {
                    if let Some(ai) = tab_context.project.audioItems.get_mut(&audio_item_key) {
                        if let Some(query) = &mut ai.query {
                            query.postPhonemeLength = after;
                        }
                    }
                }
            }
            self.depth -= 1;
        }
    }

    /// record changes and apply changes
    pub(crate) fn apply(&mut self, mut diff: Diff, tab_context: &mut TabContext) {
        self.redo_stack.clear();
        match &mut diff {
            Diff::Text {
                audio_item_key,
                before: before_text,
                after: after_text,
            } => {
                if let Some(ai) = tab_context.project.audioItems.get_mut(audio_item_key) {
                    *before_text = ai.text.clone();
                    ai.text = after_text.clone();
                }
            }
            Diff::Pitch {
                audio_item_key,
                before,
                after,
            } => {
                if let Some(ai) = tab_context.project.audioItems.get_mut(audio_item_key) {
                    if let Some(query) = &mut ai.query {
                        *before = query.pitchScale;
                        query.pitchScale = *after;
                    }
                }
            }
            Diff::Speed {
                audio_item_key,
                before,
                after,
            } => {
                if let Some(ai) = tab_context.project.audioItems.get_mut(audio_item_key) {
                    if let Some(query) = &mut ai.query {
                        *before = query.speedScale;
                        query.speedScale = *after;
                    }
                }
            }
            Diff::Intonation {
                audio_item_key,
                before,
                after,
            } => {
                if let Some(ai) = tab_context.project.audioItems.get_mut(audio_item_key) {
                    if let Some(query) = &mut ai.query {
                        *before = query.intonationScale;
                        query.intonationScale = *after;
                    }
                }
            }
            Diff::Volume {
                audio_item_key,
                before,
                after,
            } => {
                if let Some(ai) = tab_context.project.audioItems.get_mut(audio_item_key) {
                    if let Some(query) = &mut ai.query {
                        *before = query.volumeScale;
                        query.volumeScale = *after;
                    }
                }
            }
            Diff::PrePhonemeLength {
                audio_item_key,
                before,
                after,
            } => {
                if let Some(ai) = tab_context.project.audioItems.get_mut(audio_item_key) {
                    if let Some(query) = &mut ai.query {
                        *before = query.prePhonemeLength;
                        query.prePhonemeLength = *after;
                    }
                }
            }
            Diff::PostPhonemeLength {
                audio_item_key,
                before,
                after,
            } => {
                if let Some(ai) = tab_context.project.audioItems.get_mut(audio_item_key) {
                    if let Some(query) = &mut ai.query {
                        *before = query.postPhonemeLength;
                        query.postPhonemeLength = *after;
                    }
                }
            }
        }
        self.depth = 0;
        self.unsquashed_buffer.push(diff);
    }

    /// 圧縮前バッファの内容を圧縮し,履歴に追加する.
    pub(crate) fn commit(&mut self) {
        // 後ろから種類が一致しないものを見つけたら　そこで止める.

        if let Some((first, last)) = self
            .unsquashed_buffer
            .first()
            .zip(self.unsquashed_buffer.last())
        {
            match (first.clone(), last.clone()) {
                (
                    Diff::Intonation {
                        audio_item_key,
                        before,
                        after: _,
                    },
                    Diff::Intonation {
                        audio_item_key: b,
                        before: _,
                        after,
                    },
                ) if audio_item_key == b => {
                    //圧縮するので圧縮前バッファを初期化.
                    self.unsquashed_buffer.clear();
                    self.undo_stack.push(Diff::Intonation {
                        audio_item_key,
                        before,
                        after,
                    });
                }
                (
                    Diff::Volume {
                        audio_item_key,
                        before,
                        after: _,
                    },
                    Diff::Volume {
                        audio_item_key: b,
                        before: _,
                        after,
                    },
                ) if audio_item_key == b => {
                    //圧縮するので圧縮前バッファを初期化.
                    self.unsquashed_buffer.clear();
                    self.undo_stack.push(Diff::Volume {
                        audio_item_key,
                        before,
                        after,
                    });
                }
                (
                    Diff::Pitch {
                        audio_item_key,
                        before,
                        after: _,
                    },
                    Diff::Pitch {
                        audio_item_key: b,
                        before: _,
                        after,
                    },
                ) if audio_item_key == b => {
                    //圧縮するので圧縮前バッファを初期化.
                    self.unsquashed_buffer.clear();

                    self.undo_stack.push(Diff::Pitch {
                        audio_item_key,
                        before,
                        after,
                    });
                }
                (
                    Diff::Speed {
                        audio_item_key,
                        before,
                        after: _,
                    },
                    Diff::Speed {
                        audio_item_key: b,
                        before: _,
                        after,
                    },
                ) if audio_item_key == b => {
                    //圧縮するので圧縮前バッファを初期化.
                    self.unsquashed_buffer.clear();

                    self.undo_stack.push(Diff::Speed {
                        audio_item_key,
                        before,
                        after,
                    });
                }
                (
                    Diff::PrePhonemeLength {
                        audio_item_key,
                        before,
                        after: _,
                    },
                    Diff::PrePhonemeLength {
                        audio_item_key: b,
                        before: _,
                        after,
                    },
                ) if audio_item_key == b => {
                    //圧縮するので圧縮前バッファを初期化.
                    self.unsquashed_buffer.clear();

                    self.undo_stack.push(Diff::PrePhonemeLength {
                        audio_item_key,
                        before,
                        after,
                    });
                }
                (
                    Diff::PostPhonemeLength {
                        audio_item_key,
                        before,
                        after: _,
                    },
                    Diff::PostPhonemeLength {
                        audio_item_key: b,
                        before: _,
                        after,
                    },
                ) if audio_item_key == b => {
                    //圧縮するので圧縮前バッファを初期化.
                    self.unsquashed_buffer.clear();

                    self.undo_stack.push(Diff::PostPhonemeLength {
                        audio_item_key,
                        before,
                        after,
                    });
                }
                (
                    Diff::Text {
                        audio_item_key,
                        before,
                        after: _,
                    },
                    Diff::Text {
                        audio_item_key: b,
                        before: _,
                        after,
                    },
                ) if audio_item_key == b => {
                    //圧縮するので圧縮前バッファを初期化.
                    self.unsquashed_buffer.clear();

                    self.undo_stack.push(Diff::Text {
                        audio_item_key,
                        before,
                        after,
                    });
                }
                (a, b) => {
                    eprintln!("not supported to squash {:?} {:?}", a, b);
                    self.undo_stack.extend_from_slice(&self.unsquashed_buffer);
                    self.unsquashed_buffer.clear();
                }
            }
        } else {
            println!("Unsquahed buffer is empty.");
        }
    }
    pub(crate) fn build_view(&self) -> Column<crate::Message, Renderer> {
        let build_text = |diff: &Diff, depth: usize, id: usize| {
            if id == 0 {
                Text::new(format!("{} 最新", if id == depth { "*" } else { "" }))
            } else {
                Text::new(match diff {
                    Diff::Text {
                        audio_item_key: _,
                        before,
                        after,
                    } => format!(
                        "{} テキスト編集　{} -> {}",
                        if depth == id { "*" } else { "" },
                        before,
                        after
                    ),
                    Diff::Pitch {
                        audio_item_key: _,
                        before,
                        after,
                    } => format!(
                        "{} 音高変更　{:.2}-> {:.2}",
                        if depth == id { "*" } else { "" },
                        before,
                        after
                    ),
                    Diff::Speed {
                        audio_item_key: _,
                        before,
                        after,
                    } => format!(
                        "{} 話速変更　{:.2}-> {:.2}",
                        if depth == id { "*" } else { "" },
                        before,
                        after
                    ),
                    Diff::Intonation {
                        audio_item_key: _,
                        before,
                        after,
                    } => format!(
                        "{} 抑揚変更　{:.2}-> {:.2}",
                        if depth == id { "*" } else { "" },
                        before,
                        after
                    ),
                    Diff::Volume {
                        audio_item_key: _,
                        before,
                        after,
                    } => format!(
                        "{} 音量変更　{:.2}-> {:.2}",
                        if depth == id { "*" } else { "" },
                        before,
                        after
                    ),
                    Diff::PrePhonemeLength {
                        audio_item_key: _,
                        before,
                        after,
                    } => format!(
                        "{} 開始無音変更　{:.2}-> {:.2}",
                        if depth == id { "*" } else { "" },
                        before,
                        after
                    ),
                    Diff::PostPhonemeLength {
                        audio_item_key: _,
                        before,
                        after,
                    } => format!(
                        "{} 終了無音変更　{:.2}-> {:.2}",
                        if depth == id { "*" } else { "" },
                        before,
                        after
                    ),
                })
            }
        };
        let once = std::iter::once(DUMMY.get_or_init(|| Diff::Text {
            audio_item_key: "dummy".to_owned(),
            before: "dummy".to_owned(),
            after: "dummy".to_owned(),
        }));
        once.chain(self.redo_stack.iter().chain(self.undo_stack.iter().rev()))
            .enumerate()
            .fold(Column::new(), |column, (id, diff)| {
                column.push(build_text(diff, self.depth, id))
            })
    }
}

static DUMMY: once_cell::sync::OnceCell<Diff> = once_cell::sync::OnceCell::new();
