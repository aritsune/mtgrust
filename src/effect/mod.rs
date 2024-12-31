pub enum TargetNumber {
    One,
    AnyNumber,
    Number(usize),
}

pub enum EffectObject {
    Target(TargetNumber),
    All,
    AnyNumber,
    Choice,
}

pub enum DamageSource {
    This,
    Other,
}

pub enum DamageTargetType {
    Player,
    Creature,
    Planeswalker,
    Battle,
    Any,
}

pub enum DestroyTargetType {
    Creature,
    Planeswalker,
    Battle,
    Artifact,
    Enchantment,
    Land,
    Permanent,
    NonLandPermanent,
}

pub enum OneShotEffectType {
    DealDamage {
        damage: usize,
        source: DamageSource,
        objects: Vec<(EffectObject, DamageTargetType)>,
    },
    Counterspell {
        objects: Vec<EffectObject>,
    },
    Destroy {
        objects: Vec<(EffectObject, DestroyTargetType)>,
    },
}

pub enum Effect {
    OneShot { effect_type: OneShotEffectType },
    Continuous,
    Multiple(Vec<Effect>),
}

pub fn test() {
    let wrath = Effect::OneShot {
        effect_type: OneShotEffectType::Destroy {
            objects: vec![(EffectObject::All, DestroyTargetType::Creature)],
        },
    };
}

