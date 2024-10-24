use indexmap::IndexMap;
use std::ops::Add;

#[derive(Debug, PartialEq, Clone)]
pub enum NenyrAnimationKind {
    Fraction,
    Progressive,
    Transitive,
    None,
}

#[derive(Debug, PartialEq, Clone)]
pub enum NenyrSubAnimationKind {
    Fraction,
    Progressive,
    From,
    Halfway,
    To,
}

#[derive(Debug, PartialEq, Clone)]
pub enum NenyrKeyframe {
    Fraction {
        stops: Vec<f64>,
        properties: IndexMap<String, String>,
    },
    Progressive(IndexMap<String, String>),
    From(IndexMap<String, String>),
    Halfway(IndexMap<String, String>),
    To(IndexMap<String, String>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct NenyrAnimation {
    animation_name: String,
    keyframe: Vec<NenyrKeyframe>,
    progressive_count: Option<i64>,
    kind: Option<NenyrAnimationKind>,
}

impl NenyrAnimation {
    pub fn new(animation_name: String) -> Self {
        Self {
            kind: None,
            animation_name,
            keyframe: Vec::new(),
            progressive_count: None,
        }
    }

    pub(crate) fn get_animation_kind(&self) -> NenyrAnimationKind {
        self.kind.clone().unwrap_or(NenyrAnimationKind::None)
    }

    pub(crate) fn increment_progressive_count(&mut self) {
        if let Some(count) = &self.progressive_count {
            self.progressive_count = Some(count.add(1));
        } else if self.progressive_count == None {
            self.progressive_count = Some(1);
        }
    }

    pub(crate) fn set_animation_kind(&mut self, current_kind: NenyrAnimationKind) -> bool {
        match &self.kind {
            None => {
                self.kind = Some(current_kind);
            }
            Some(kind) if kind != &current_kind => {
                return false;
            }
            _ => {}
        }

        true
    }

    pub(crate) fn add_animation_keyframe(
        &mut self,
        sub_kind: &NenyrSubAnimationKind,
        stops: &Option<Vec<f64>>,
        keyframe: IndexMap<String, String>,
    ) {
        match sub_kind {
            NenyrSubAnimationKind::Fraction => {
                let stops: Vec<f64> = stops.clone().unwrap_or(vec![]);
                let fraction = NenyrKeyframe::Fraction {
                    stops,
                    properties: keyframe,
                };

                self.keyframe.push(fraction);
            }
            NenyrSubAnimationKind::Progressive => {
                self.keyframe.push(NenyrKeyframe::Progressive(keyframe));
            }
            NenyrSubAnimationKind::From => {
                self.keyframe.push(NenyrKeyframe::From(keyframe));
            }
            NenyrSubAnimationKind::Halfway => {
                self.keyframe.push(NenyrKeyframe::Halfway(keyframe));
            }
            NenyrSubAnimationKind::To => {
                self.keyframe.push(NenyrKeyframe::To(keyframe));
            }
        }
    }
}

#[cfg(test)]
mod tests {}
