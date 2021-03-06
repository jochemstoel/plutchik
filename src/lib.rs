//!  
//!  Typification of Robert Plutchik's [Wheel of Emotions](https://en.wikipedia.org/wiki/Contrasting_and_categorization_of_emotions#/media/File:Plutchik-wheel.svg).
//!


extern crate rand;


use std::f32::consts::PI;
use std::ops::Deref;


/// Each of the emotions portrayed on Plutchik's emotion wheel.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Emotion {
    Ecstasy,
    Joy,
    Serenity,
    Love,
    Admiration,
    Trust,
    Acceptance,
    Submission,
    Terror,
    Fear,
    Apprehension,
    Awe,
    Amazement,
    Surprise,
    Distraction,
    Disapproval,
    Grief,
    Sadness,
    Pensiveness,
    Remorse,
    Loathing,
    Disgust,
    Boredom,
    Contempt,
    Rage,
    Anger,
    Annoyance,
    Aggressiveness,
    Vigilance,
    Anticipation,
    Interest,
    Optimism,
}


/// Representation of an emotion in the form of a Plutchik Wheel.
/// Useful for emotional tagging and aesthetic comparisons.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Wheel {
    pub radians: f32,
    pub weight: f32,
}


/// The full list of Plutchik's labelled emotions.
pub const EMOTIONS: &'static [Emotion; 32] = &[
    Emotion::Ecstasy,
    Emotion::Joy,
    Emotion::Serenity,
    Emotion::Love,
    Emotion::Admiration,
    Emotion::Trust,
    Emotion::Acceptance,
    Emotion::Submission,
    Emotion::Terror,
    Emotion::Fear,
    Emotion::Apprehension,
    Emotion::Awe,
    Emotion::Amazement,
    Emotion::Surprise,
    Emotion::Distraction,
    Emotion::Disapproval,
    Emotion::Grief,
    Emotion::Sadness,
    Emotion::Pensiveness,
    Emotion::Remorse,
    Emotion::Loathing,
    Emotion::Disgust,
    Emotion::Boredom,
    Emotion::Contempt,
    Emotion::Rage,
    Emotion::Anger,
    Emotion::Annoyance,
    Emotion::Aggressiveness,
    Emotion::Vigilance,
    Emotion::Anticipation,
    Emotion::Interest,
    Emotion::Optimism,
];


pub mod wheels {
    //!  const `Wheel`s associated with Plutchik's labelled emotions.

    use std::f32::consts::PI;
    use super::Wheel;

    /// A macro for generating Emotion `Wheel` constants.
    macro_rules! wheel {
        ($name:ident { $radians:expr, $weight:expr }) => {
            pub const $name: &'static Wheel = &Wheel { radians: $radians, weight: $weight };
        };
    }

    wheel!(ECSTASY        { PI * 0.5    , 0.8 });
    wheel!(JOY            { PI * 0.5    , 0.5 });
    wheel!(SERENITY       { PI * 0.5    , 0.2 });
    wheel!(LOVE           { PI * 0.375  , 0.5 });
    wheel!(ADMIRATION     { PI * 0.25   , 0.8 });
    wheel!(TRUST          { PI * 0.25   , 0.5 });
    wheel!(ACCEPTANCE     { PI * 0.25   , 0.2 });
    wheel!(SUBMISSION     { PI * 0.125  , 0.5 });
    wheel!(TERROR         { PI * 0.0    , 0.8 });
    wheel!(FEAR           { PI * 0.0    , 0.5 });
    wheel!(APPREHENSION   { PI * 0.0    , 0.2 });
    wheel!(AWE            { PI * -0.125 , 0.5 });
    wheel!(AMAZEMENT      { PI * -0.25  , 0.8 });
    wheel!(SURPRISE       { PI * -0.25  , 0.5 });
    wheel!(DISTRACTION    { PI * -0.25  , 0.2 });
    wheel!(DISAPPROVAL    { PI * -0.375 , 0.5 });
    wheel!(GRIEF          { PI * -0.5   , 0.8 });
    wheel!(SADNESS        { PI * -0.5   , 0.5 });
    wheel!(PENSIVENESS    { PI * -0.5   , 0.2 });
    wheel!(REMORSE        { PI * -0.625 , 0.5 });
    wheel!(LOATHING       { PI * -0.75  , 0.8 });
    wheel!(DISGUST        { PI * -0.75  , 0.5 });
    wheel!(BOREDOM        { PI * -0.75  , 0.2 });
    wheel!(CONTEMPT       { PI * -0.875 , 0.5 });
    wheel!(RAGE           { PI * -1.0   , 0.8 });
    wheel!(ANGER          { PI * -1.0   , 0.5 });
    wheel!(ANNOYANCE      { PI * -1.0   , 0.2 });
    wheel!(AGGRESSIVENESS { PI * 0.875  , 0.5 });
    wheel!(VIGILANCE      { PI * 0.75   , 0.8 });
    wheel!(ANTICIPATION   { PI * 0.75   , 0.5 });
    wheel!(INTEREST       { PI * 0.75   , 0.2 });
    wheel!(OPTIMISM       { PI * 0.625  , 0.5 });
}


impl Deref for Emotion {
    type Target = Wheel;
    fn deref<'a>(&'a self) -> &'a Wheel {
        match *self {
            Emotion::Ecstasy        => wheels::ECSTASY,
            Emotion::Joy            => wheels::JOY,
            Emotion::Serenity       => wheels::SERENITY,
            Emotion::Love           => wheels::LOVE,
            Emotion::Admiration     => wheels::ADMIRATION,
            Emotion::Trust          => wheels::TRUST,
            Emotion::Acceptance     => wheels::ACCEPTANCE,
            Emotion::Submission     => wheels::SUBMISSION,
            Emotion::Terror         => wheels::TERROR,
            Emotion::Fear           => wheels::FEAR,
            Emotion::Apprehension   => wheels::APPREHENSION,
            Emotion::Awe            => wheels::AWE,
            Emotion::Amazement      => wheels::AMAZEMENT,
            Emotion::Surprise       => wheels::SURPRISE,
            Emotion::Distraction    => wheels::DISTRACTION,
            Emotion::Disapproval    => wheels::DISAPPROVAL,
            Emotion::Grief          => wheels::GRIEF,
            Emotion::Sadness        => wheels::SADNESS,
            Emotion::Pensiveness    => wheels::PENSIVENESS,
            Emotion::Remorse        => wheels::REMORSE,
            Emotion::Loathing       => wheels::LOATHING,
            Emotion::Disgust        => wheels::DISGUST,
            Emotion::Boredom        => wheels::BOREDOM,
            Emotion::Contempt       => wheels::CONTEMPT,
            Emotion::Rage           => wheels::RAGE,
            Emotion::Anger          => wheels::ANGER,
            Emotion::Annoyance      => wheels::ANNOYANCE,
            Emotion::Aggressiveness => wheels::AGGRESSIVENESS,
            Emotion::Vigilance      => wheels::VIGILANCE,
            Emotion::Anticipation   => wheels::ANTICIPATION,
            Emotion::Interest       => wheels::INTEREST,
            Emotion::Optimism       => wheels::OPTIMISM,
        }
    }
}


impl Wheel {

    /// Construct the mean (average) emotion wheel given a number of other emotions.
    pub fn mean<T>(emotions: &[T]) -> Wheel where T: Deref<Target=Wheel> {
        match emotions.len() {
            0   => Wheel { radians: 0.0, weight: 0.0, },
            len => {
                let mut vecs = emotions.iter().map(|emotion| {
                    let vec: [f32; 2] = (**emotion).into();
                    vec
                });
                let sum_vec = vecs.next().map(|vec| {
                    vecs.fold(vec, |sum, vec| [sum[0] + vec[0], sum[1] + vec[1]])
                }).unwrap();
                [sum_vec[0] / len as f32, sum_vec[1] / len as f32].into()
            },
        }
    }

    /// Compare two Wheels and return the result as a difference weight where 0 is the minimum
    /// distance and 1 is the maximum. Determine the difference as the magnitude of the vector
    /// that separates both points on the wheel.
    pub fn difference(&self, other: &Wheel) -> f32 {
        let self_vec: [f32; 2]  = self.into();
        let other_vec: [f32; 2] = other.into();
        let diff_vec = [self_vec[0] - other_vec[0], self_vec[1] - other_vec[1]];
        magnitude(&diff_vec) / 2.0
    }

    /// Return the closest matching `Emotion` variant to the current state of the Wheel.
    pub fn closest_emotion(&self) -> Emotion {
        let mut emotions = EMOTIONS.iter();
        let (&emotion, _) = emotions.next().map(|emotion| {
            let diff = emotion.difference(self);
            emotions.fold((emotion, diff), |(closest, closest_diff), emotion| {
                let diff = emotion.difference(self);
                if diff < closest_diff { (emotion, diff) } else { (closest, closest_diff) }
            })
        }).unwrap();
        emotion
    }

    /// Return the closest `n` number of emotions to the current state of the wheel.
    pub fn closest_emotions(&self, n: usize) -> Vec<Emotion> {
        match n {
            0 => Vec::new(),
            1 => vec![self.closest_emotion()],
            _ => {
                let mut emotions = *EMOTIONS;
                emotions.sort_by(|a, b| {
                    a.difference(self).partial_cmp(&b.difference(self)).unwrap()
                });
                emotions.iter().map(|&emotion| emotion).take(n).collect()
            },
        }
    }

    /// Return the wheel's opposite.
    pub fn opposite(&self) -> Wheel {
        const TAU: f32 = PI * 2.0;
        let radians = (self.radians + PI) % TAU;
        Wheel { radians: radians, ..*self }
    }

}


fn magnitude(vec: &[f32; 2]) -> f32 {
    (vec[0].powf(2.0) + vec[1].powf(2.0)).sqrt()
}


impl rand::Rand for Emotion {
    fn rand<R: rand::Rng>(rng: &mut R) -> Emotion {
        EMOTIONS[rng.gen_range(0, EMOTIONS.len())]
    }
}


impl rand::Rand for Wheel {
    fn rand<R: rand::Rng>(rng: &mut R) -> Wheel {
        Wheel {
            radians: rng.gen::<f32>() * PI * 2.0,
            weight: rng.gen::<f32>(),
        }
    }
}


impl<'a> Into<[f32; 2]> for &'a Wheel {
    fn into(self) -> [f32; 2] {
        [self.radians.cos() * self.weight, self.radians.sin() * self.weight]
    }
}

impl<'a> Into<[f32; 2]> for Wheel {
    fn into(self) -> [f32; 2] {
        [self.radians.cos() * self.weight, self.radians.sin() * self.weight]
    }
}


impl<'a> From<[f32; 2]> for Wheel {
    fn from(vec: [f32; 2]) -> Wheel {
        let weight = magnitude(&vec);
        let radians = vec[1].atan2(vec[0]);
        Wheel { radians: radians, weight: weight }
    }
}

