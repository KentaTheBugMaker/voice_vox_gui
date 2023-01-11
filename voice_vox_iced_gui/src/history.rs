use iced::{
    widget::{Column, Text},
    Renderer,
};

use crate::TabContext;
#[derive(Debug, Clone)]
pub enum Diff {
    TextChanged {
        audio_item_key: String,
        before: String,
        after: String,
    },
    PitchChanged {
        audio_item_key: String,
        before: f64,
        after: f64,
    },
    SpeedChanged {
        audio_item_key: String,
        before: f64,
        after: f64,
    },
    IntonationChanged {
        audio_item_key: String,
        before: f64,
        after: f64,
    },
    VolumeChanged {
        audio_item_key: String,
        before: f64,
        after: f64,
    },
    PrePhonemeLengthChanged {
        audio_item_key: String,
        before: f64,
        after: f64,
    },
    PostPhonemeLengthChanged {
        audio_item_key: String,
        before: f64,
        after: f64,
    },
}

#[derive(Debug, Clone)]
pub struct History {
    buffer: Vec<Diff>,
    history_position: usize,

    unsquashed_buffer: Vec<Diff>,
}
impl History {
    pub(crate) fn new() -> Self {
        Self {
            buffer: vec![],
            history_position: 0,
            unsquashed_buffer: Vec::new(),
        }
    }
    /// undo
    pub(crate) fn undo(&mut self, tab_context: &mut TabContext) {
        if self.history_position > 0 {
            self.history_position -= 1;
        } else {
            eprintln!("pumping down to underground!");
        }
        if let Some(diff) = self.buffer.get(self.history_position) {
            match diff {
                Diff::TextChanged {
                    audio_item_key,
                    before,
                    after: _,
                } => {
                    if let Some(ai) = tab_context.project.audioItems.get_mut(audio_item_key) {
                        ai.text = before.clone();
                    }
                }
                Diff::PitchChanged {
                    audio_item_key,
                    before,
                    after: _,
                } => {
                    if let Some(ai) = tab_context.project.audioItems.get_mut(audio_item_key) {
                        if let Some(query) = &mut ai.query {
                            query.pitchScale = *before;
                        }
                    }
                }
                Diff::SpeedChanged {
                    audio_item_key,
                    before,
                    after: _,
                } => {
                    if let Some(ai) = tab_context.project.audioItems.get_mut(audio_item_key) {
                        if let Some(query) = &mut ai.query {
                            query.speedScale = *before;
                        }
                    }
                }
                Diff::IntonationChanged {
                    audio_item_key,
                    before,
                    after: _,
                } => {
                    if let Some(ai) = tab_context.project.audioItems.get_mut(audio_item_key) {
                        if let Some(query) = &mut ai.query {
                            query.intonationScale = *before;
                        }
                    }
                }
                Diff::VolumeChanged {
                    audio_item_key,
                    before,
                    after: _,
                } => {
                    if let Some(ai) = tab_context.project.audioItems.get_mut(audio_item_key) {
                        if let Some(query) = &mut ai.query {
                            query.volumeScale = *before;
                        }
                    }
                }
                Diff::PrePhonemeLengthChanged {
                    audio_item_key,
                    before,
                    after: _,
                } => {
                    if let Some(ai) = tab_context.project.audioItems.get_mut(audio_item_key) {
                        if let Some(query) = &mut ai.query {
                            query.prePhonemeLength = *before;
                        }
                    }
                }
                Diff::PostPhonemeLengthChanged {
                    audio_item_key,
                    before,
                    after: _,
                } => {
                    if let Some(ai) = tab_context.project.audioItems.get_mut(audio_item_key) {
                        if let Some(query) = &mut ai.query {
                            query.postPhonemeLength = *before;
                        }
                    }
                }
            }
        }
    }
    /// redo
    pub(crate) fn redo(&mut self, tab_context: &mut TabContext) {
        if self.buffer.is_empty() {
            return;
        }
        if self.history_position < self.buffer.len() - 1 {
            self.history_position += 1;
        } else {
            eprintln!("pumping up to heaven");
        }
        if let Some(diff) = self.buffer.get(self.history_position) {
            match diff {
                Diff::TextChanged {
                    audio_item_key,
                    before: _,
                    after,
                } => {
                    if let Some(ai) = tab_context.project.audioItems.get_mut(audio_item_key) {
                        ai.text = after.clone();
                    }
                }
                Diff::PitchChanged {
                    audio_item_key,
                    before: _,
                    after,
                } => {
                    if let Some(ai) = tab_context.project.audioItems.get_mut(audio_item_key) {
                        if let Some(query) = &mut ai.query {
                            query.pitchScale = *after;
                        }
                    }
                }
                Diff::SpeedChanged {
                    audio_item_key,
                    before: _,
                    after,
                } => {
                    if let Some(ai) = tab_context.project.audioItems.get_mut(audio_item_key) {
                        if let Some(query) = &mut ai.query {
                            query.speedScale = *after;
                        }
                    }
                }
                Diff::IntonationChanged {
                    audio_item_key,
                    before: _,
                    after,
                } => {
                    if let Some(ai) = tab_context.project.audioItems.get_mut(audio_item_key) {
                        if let Some(query) = &mut ai.query {
                            query.intonationScale = *after;
                        }
                    }
                }
                Diff::VolumeChanged {
                    audio_item_key,
                    before: _,
                    after,
                } => {
                    if let Some(ai) = tab_context.project.audioItems.get_mut(audio_item_key) {
                        if let Some(query) = &mut ai.query {
                            query.volumeScale = *after;
                        }
                    }
                }
                Diff::PrePhonemeLengthChanged {
                    audio_item_key,
                    before: _,
                    after,
                } => {
                    if let Some(ai) = tab_context.project.audioItems.get_mut(audio_item_key) {
                        if let Some(query) = &mut ai.query {
                            query.prePhonemeLength = *after;
                        }
                    }
                }
                Diff::PostPhonemeLengthChanged {
                    audio_item_key,
                    before: _,
                    after,
                } => {
                    if let Some(ai) = tab_context.project.audioItems.get_mut(audio_item_key) {
                        if let Some(query) = &mut ai.query {
                            query.postPhonemeLength = *after;
                        }
                    }
                }
            }
        }
    }

    /// record changes and apply changes
    pub(crate) fn apply(&mut self, mut diff: Diff, tab_context: &mut TabContext) {
        println!("hp= {} len= {}", self.history_position, self.buffer.len());
        if self.history_position + 1 < self.buffer.len() {
            self.buffer.truncate(self.history_position);
        }
        match &mut diff {
            Diff::TextChanged {
                audio_item_key,
                before: before_text,
                after: after_text,
            } => {
                if let Some(ai) = tab_context.project.audioItems.get_mut(audio_item_key) {
                    *before_text = ai.text.clone();
                    ai.text = after_text.clone();
                }
            }
            Diff::PitchChanged {
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
            Diff::SpeedChanged {
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
            Diff::IntonationChanged {
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
            Diff::VolumeChanged {
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
            Diff::PrePhonemeLengthChanged {
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
            Diff::PostPhonemeLengthChanged {
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
                    Diff::IntonationChanged {
                        audio_item_key,
                        before,
                        after: _,
                    },
                    Diff::IntonationChanged {
                        audio_item_key: b,
                        before: _,
                        after,
                    },
                ) if audio_item_key == b => {
                    //圧縮するので圧縮前バッファを初期化.
                    self.unsquashed_buffer.clear();
                    self.history_position = self.buffer.len();
                    self.buffer.push(Diff::IntonationChanged {
                        audio_item_key,
                        before,
                        after,
                    });
                }
                (
                    Diff::VolumeChanged {
                        audio_item_key,
                        before,
                        after: _,
                    },
                    Diff::VolumeChanged {
                        audio_item_key: b,
                        before: _,
                        after,
                    },
                ) if audio_item_key == b => {
                    //圧縮するので圧縮前バッファを初期化.
                    self.unsquashed_buffer.clear();
                    self.history_position = self.buffer.len();
                    self.buffer.push(Diff::VolumeChanged {
                        audio_item_key,
                        before,
                        after,
                    });
                }
                (
                    Diff::PitchChanged {
                        audio_item_key,
                        before,
                        after: _,
                    },
                    Diff::PitchChanged {
                        audio_item_key: b,
                        before: _,
                        after,
                    },
                ) if audio_item_key == b => {
                    //圧縮するので圧縮前バッファを初期化.
                    self.unsquashed_buffer.clear();
                    self.history_position = self.buffer.len();
                    self.buffer.push(Diff::PitchChanged {
                        audio_item_key,
                        before,
                        after,
                    });
                }
                (
                    Diff::SpeedChanged {
                        audio_item_key,
                        before,
                        after: _,
                    },
                    Diff::SpeedChanged {
                        audio_item_key: b,
                        before: _,
                        after,
                    },
                ) if audio_item_key == b => {
                    //圧縮するので圧縮前バッファを初期化.
                    self.unsquashed_buffer.clear();
                    self.history_position = self.buffer.len();
                    self.buffer.push(Diff::SpeedChanged {
                        audio_item_key,
                        before,
                        after,
                    });
                }
                (
                    Diff::PrePhonemeLengthChanged {
                        audio_item_key,
                        before,
                        after: _,
                    },
                    Diff::PrePhonemeLengthChanged {
                        audio_item_key: b,
                        before: _,
                        after,
                    },
                ) if audio_item_key == b => {
                    //圧縮するので圧縮前バッファを初期化.
                    self.unsquashed_buffer.clear();
                    self.history_position = self.buffer.len();
                    self.buffer.push(Diff::PrePhonemeLengthChanged {
                        audio_item_key,
                        before,
                        after,
                    });
                }
                (
                    Diff::PostPhonemeLengthChanged {
                        audio_item_key,
                        before,
                        after: _,
                    },
                    Diff::PostPhonemeLengthChanged {
                        audio_item_key: b,
                        before: _,
                        after,
                    },
                ) if audio_item_key == b => {
                    //圧縮するので圧縮前バッファを初期化.
                    self.unsquashed_buffer.clear();
                    self.history_position = self.buffer.len();
                    self.buffer.push(Diff::PostPhonemeLengthChanged {
                        audio_item_key,
                        before,
                        after,
                    });
                }
                (
                    Diff::TextChanged {
                        audio_item_key,
                        before,
                        after: _,
                    },
                    Diff::TextChanged {
                        audio_item_key: b,
                        before: _,
                        after,
                    },
                ) if audio_item_key == b => {
                    //圧縮するので圧縮前バッファを初期化.
                    self.unsquashed_buffer.clear();
                    self.history_position = self.buffer.len();
                    self.buffer.push(Diff::TextChanged {
                        audio_item_key,
                        before,
                        after,
                    });
                }
                (a, b) => {
                    eprintln!("not supported to squash {:?} {:?}", a, b);
                    self.history_position = self.buffer.len();
                    self.buffer.extend_from_slice(&self.unsquashed_buffer);
                    self.unsquashed_buffer.clear();
                }
            }
        } else {
            println!("Unsquahed buffer is empty.");
        }
    }
    pub(crate) fn build_view(&self) -> Column<crate::Message, Renderer> {
        self.buffer
            .iter()
            .enumerate()
            .rev()
            .fold(Column::new(), |column, (idx, diff)| {
                column.push(Text::new(match diff {
                    Diff::TextChanged {
                        audio_item_key: _,
                        before,
                        after,
                    } => format!(
                        "{} テキスト編集　{} -> {}",
                        if idx == self.history_position {
                            "*"
                        } else {
                            ""
                        },
                        before,
                        after
                    ),
                    Diff::PitchChanged {
                        audio_item_key: _,
                        before,
                        after,
                    } => format!(
                        "{} 音高変更　{:.2}-> {:.2}",
                        if idx == self.history_position {
                            "*"
                        } else {
                            ""
                        },
                        before,
                        after
                    ),
                    Diff::SpeedChanged {
                        audio_item_key: _,
                        before,
                        after,
                    } => format!(
                        "{} 話速変更　{:.2}-> {:.2}",
                        if idx == self.history_position {
                            "*"
                        } else {
                            ""
                        },
                        before,
                        after
                    ),
                    Diff::IntonationChanged {
                        audio_item_key: _,
                        before,
                        after,
                    } => format!(
                        "{} 抑揚変更　{:.2}-> {:.2}",
                        if idx == self.history_position {
                            "*"
                        } else {
                            ""
                        },
                        before,
                        after
                    ),
                    Diff::VolumeChanged {
                        audio_item_key: _,
                        before,
                        after,
                    } => format!(
                        "{} 音量変更　{:.2}-> {:.2}",
                        if idx == self.history_position {
                            "*"
                        } else {
                            ""
                        },
                        before,
                        after
                    ),
                    Diff::PrePhonemeLengthChanged {
                        audio_item_key: _,
                        before,
                        after,
                    } => format!(
                        "{} 開始無音変更　{:.2}-> {:.2}",
                        if idx == self.history_position {
                            "*"
                        } else {
                            ""
                        },
                        before,
                        after
                    ),
                    Diff::PostPhonemeLengthChanged {
                        audio_item_key: _,
                        before,
                        after,
                    } => format!(
                        "{}終了無音変更　{:.2}-> {:.2}",
                        if idx == self.history_position {
                            "*"
                        } else {
                            ""
                        },
                        before,
                        after
                    ),
                }))
            })
    }
}
