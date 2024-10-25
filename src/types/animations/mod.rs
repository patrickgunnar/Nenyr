use indexmap::IndexMap;
use std::ops::Add;

/// Represents the kind of animation in Nenyr.
///
/// This enum defines the different types of animations that can be applied in
/// the Nenyr framework. These include:
///
/// - `Fraction`: Represents an animation with fractional stops at specific intervals.
/// - `Progressive`: Represents an animation that progresses sequentially from start to end.
/// - `Transitive`: A smooth transitioning animation.
/// - `None`: Indicates that no animation kind has been applied.
#[derive(Debug, PartialEq, Clone)]
pub enum NenyrAnimationKind {
    Fraction,
    Progressive,
    Transitive,
    None,
}

/// Represents subtypes of keyframes within a Nenyr animation.
///
/// This enum describes the various keyframe types that can be used for an
/// animation, helping to further define the behavior and timing of animations:
///
/// - `Fraction`: Keyframes are tied to fractional intervals (percentages).
/// - `Progressive`: Keyframes that occur progressively over time.
/// - `From`: The starting keyframe in an animation.
/// - `Halfway`: A keyframe set at the midpoint of the animation.
/// - `To`: The final keyframe at the end of the animation.
#[derive(Debug, PartialEq, Clone)]
pub enum NenyrSubAnimationKind {
    Fraction,
    Progressive,
    From,
    Halfway,
    To,
}

/// Describes the keyframe structure for a Nenyr animation.
///
/// `NenyrKeyframe` is an enum that allows for different types of keyframe
/// definitions depending on the animation type:
///
/// - `Fraction`: A keyframe tied to fractional stops, with properties at each stop.
/// - `Progressive`: A progressively applied set of properties.
/// - `From`: The starting point of an animation.
/// - `Halfway`: The midpoint in the animation.
/// - `To`: The ending point in the animation.
#[derive(Debug, PartialEq, Clone)]
pub enum NenyrKeyframe {
    Fraction {
        /// Vector of fractional stops for the keyframe (e.g., 0.0 to 1.0 representing 0% to 100%).
        stops: Vec<f64>,
        /// CSS properties for the keyframe at each stop.
        properties: IndexMap<String, String>,
    },
    Progressive(IndexMap<String, String>),
    From(IndexMap<String, String>),
    Halfway(IndexMap<String, String>),
    To(IndexMap<String, String>),
}

/// The main struct representing an animation in Nenyr.
///
/// `NenyrAnimation` defines an animation consisting of keyframes, the type of animation,
/// and optionally, a progressive count which tracks how many times the progressive
/// animation has been applied.
///
/// ### Fields:
/// - `animation_name`: The name of the animation.
/// - `keyframe`: A vector of `NenyrKeyframe`s representing the various keyframes in the animation.
/// - `progressive_count`: An optional counter to track progressive keyframes.
/// - `kind`: The kind of animation, if any, applied to this animation instance.
#[derive(Debug, PartialEq, Clone)]
pub struct NenyrAnimation {
    pub animation_name: String,
    pub kind: Option<NenyrAnimationKind>,
    pub progressive_count: Option<i64>,
    pub keyframe: Vec<NenyrKeyframe>,
}

impl NenyrAnimation {
    /// Creates a new `NenyrAnimation` instance with a given animation name.
    ///
    /// This initializes the animation with an empty keyframe list, no progressive
    /// count, and no specific animation kind.
    ///
    /// ### Parameters:
    /// - `animation_name`: The name of the animation.
    ///
    /// ### Returns:
    /// A new instance of `NenyrAnimation`.
    pub fn new(animation_name: String) -> Self {
        Self {
            kind: None,
            animation_name,
            keyframe: Vec::new(),
            progressive_count: None,
        }
    }

    /// Retrieves the animation kind for this animation instance.
    ///
    /// If no kind has been set, this will return `NenyrAnimationKind::None` by default.
    ///
    /// ### Returns:
    /// The `NenyrAnimationKind` that represents the type of animation.
    pub(crate) fn get_animation_kind(&self) -> NenyrAnimationKind {
        self.kind.clone().unwrap_or(NenyrAnimationKind::None)
    }

    /// Increments the `progressive_count` for progressive animations.
    ///
    /// This function tracks how many progressive keyframes have been added to the animation.
    /// If no count exists, it will initialize the count to `1`.
    pub(crate) fn increment_progressive_count(&mut self) {
        if let Some(count) = &self.progressive_count {
            self.progressive_count = Some(count.add(1));
        } else if self.progressive_count == None {
            self.progressive_count = Some(1);
        }
    }

    /// Sets the kind of animation for this animation instance.
    ///
    /// If an animation kind has already been set and differs from the provided kind,
    /// the function will return `false`, indicating that the animation kind could
    /// not be changed.
    ///
    /// ### Parameters:
    /// - `current_kind`: The new `NenyrAnimationKind` to set.
    ///
    /// ### Returns:
    /// `true` if the kind was successfully set or if it matches the current kind,
    /// `false` if there was a mismatch.
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

    /// Adds a keyframe to the animation based on the sub-kind and properties.
    ///
    /// This function accepts a sub-animation kind and adds the corresponding keyframe
    /// type (`Fraction`, `Progressive`, `From`, `Halfway`, `To`) to the animation.
    ///
    /// ### Parameters:
    /// - `sub_kind`: The sub-animation kind (`NenyrSubAnimationKind`) determining the keyframe type.
    /// - `stops`: An optional vector of fractional stops, relevant only for `Fraction` keyframes.
    /// - `keyframe`: A map of CSS properties (`IndexMap<String, String>`) that define the keyframe.
    pub(crate) fn add_animation_keyframe(
        &mut self,
        sub_kind: &NenyrSubAnimationKind,
        stops: &Option<Vec<f64>>,
        keyframe: IndexMap<String, String>,
    ) {
        match sub_kind {
            NenyrSubAnimationKind::Fraction => {
                let stops: Vec<f64> = stops.clone().unwrap_or(vec![0.0]);
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
mod tests {
    use indexmap::IndexMap;

    use crate::types::animations::{
        NenyrAnimation, NenyrAnimationKind, NenyrKeyframe, NenyrSubAnimationKind,
    };

    #[test]
    fn test_nenyr_animation_kind_values() {
        assert_eq!(NenyrAnimationKind::Fraction, NenyrAnimationKind::Fraction);
        assert_eq!(NenyrAnimationKind::None, NenyrAnimationKind::None);
    }

    #[test]
    fn test_nenyr_sub_animation_kind_values() {
        assert_eq!(NenyrSubAnimationKind::From, NenyrSubAnimationKind::From);
        assert_eq!(NenyrSubAnimationKind::To, NenyrSubAnimationKind::To);
    }

    #[test]
    fn test_nenyr_animation_new() {
        let animation = NenyrAnimation::new("fade_in".to_string());

        assert_eq!(animation.animation_name, "fade_in");
        assert_eq!(animation.kind, None);
        assert!(animation.keyframe.is_empty());
        assert_eq!(animation.progressive_count, None);
    }

    #[test]
    fn test_get_animation_kind() {
        let mut animation = NenyrAnimation::new("slide_in".to_string());

        assert_eq!(animation.get_animation_kind(), NenyrAnimationKind::None);

        animation.kind = Some(NenyrAnimationKind::Fraction);
        assert_eq!(animation.get_animation_kind(), NenyrAnimationKind::Fraction);
    }

    #[test]
    fn test_increment_progressive_count() {
        let mut animation = NenyrAnimation::new("zoom_in".to_string());

        assert_eq!(animation.progressive_count, None);

        animation.increment_progressive_count();
        assert_eq!(animation.progressive_count, Some(1));

        animation.increment_progressive_count();
        assert_eq!(animation.progressive_count, Some(2));
    }

    #[test]
    fn test_set_animation_kind() {
        let mut animation = NenyrAnimation::new("fade_out".to_string());

        assert!(animation.set_animation_kind(NenyrAnimationKind::Fraction));
        assert_eq!(animation.kind, Some(NenyrAnimationKind::Fraction));

        assert!(!animation.set_animation_kind(NenyrAnimationKind::Progressive));
        assert_eq!(animation.kind, Some(NenyrAnimationKind::Fraction));

        assert!(animation.set_animation_kind(NenyrAnimationKind::Fraction));
    }

    #[test]
    fn test_add_animation_keyframe() {
        let mut animation = NenyrAnimation::new("fade".to_string());
        let mut properties = IndexMap::new();

        properties.insert("opacity".to_string(), "0".to_string());

        animation.add_animation_keyframe(&NenyrSubAnimationKind::From, &None, properties.clone());
        assert!(matches!(animation.keyframe[0], NenyrKeyframe::From(_)));

        animation.add_animation_keyframe(&NenyrSubAnimationKind::To, &None, properties.clone());
        assert!(matches!(animation.keyframe[1], NenyrKeyframe::To(_)));

        let stops = vec![0.0, 0.5, 1.0];

        animation.add_animation_keyframe(
            &NenyrSubAnimationKind::Fraction,
            &Some(stops.clone()),
            properties.clone(),
        );

        if let NenyrKeyframe::Fraction { stops: s, .. } = &animation.keyframe[2] {
            assert_eq!(*s, stops);
        } else {
            panic!("NenyrKeyframe::Fraction not added correctly");
        }
    }
}
