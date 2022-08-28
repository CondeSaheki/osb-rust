// Screen; x: 0–640, y: 0–480
// Playarea; x: 60–570, y: 55–440

struct Coordinates
{
    x: i32,
    y: i32,
}

/* OBJECTS TYPES */

enum Layer
{
    Background,
    Fail,
    Pass,
    Foreground,
}
impl Layer
{
    fn value(layer: &self) -> i32
    {
        match layer
        {
            Background => 0,
            Fail => 1,
            Pass => 2,
            Foreground => 3,
        }
    }
}

enum Origin
{
    TopLeft,
    Centre,
    CentreLeft,
    TopRight,
    BottomCentre,
    TopCentre,
    CentreRight,
    BottomLeft,
    BottomRight,
}
impl Origin
{
    fn coordinates(origin: &self) -> Coordinates
    {
        match origin
        {
            TopLeft => Coordinates { x: 0, y: 0 },
            TopCentre => Coordinates { x: 320, y: 0 },
            TopRight => Coordinates { x: 640, y: 0 },
            CentreLeft => Coordinates { x: 0, y: 240 },
            Centre => Coordinates { x: 320, y: 240 },
            CentreRight => Coordinates { x: 640, y: 240 },
            BottomLeft => Coordinates { x: 0, y: 480 },
            BottomCentre => Coordinates { x: 320, y: 480 },
            BottomRight => Coordinates { x: 640, y: 480 },
        }
    }
    fn value(origin: &self) -> i32
    {
        match origin
        {
            TopLeft => 0,
            Centre => 1,
            CentreLeft => 2,
            TopRight => 3,
            BottomCentre => 4,
            TopCentre => 5,
            CentreRight => 7,
            BottomLeft => 8,
            BottomRight => 9,
        }
    }
    fn str(origin: &self) -> str
    {
        match origin
        {
            TopLeft => "TopLeft",
            Centre => "Centre",
            CentreLeft => "CentreLeft",
            TopRight => "TopRight",
            BottomCentre => "BottomCentre",
            TopCentre => "TopCentre",
            CentreRight => "CentreRight",
            BottomLeft => "BottomLeft",
            BottomRight => "BottomRight",
        }
    }
}

enum LoopType
{
    LoopForever,
    LoopOnce,
}
impl LoopType
{
    fn value(looptype: &self) -> str
    {
        match looptype
        {
            LoopForever => "LoopForever",
            LoopOnce => "LoopOnce",
        }
    }
}

/* OBJECTS */

//Sprite,(layer),(origin),"(filepath)",(x),(y)
struct Sprite
{
    layer: Layer,
    origin: Origin,
    filepath: String,
    coordinates: Coordinates,
}
impl Sprite
{
    fn generate(obj: &self);
}

//Animation,(layer),(origin),"(filepath)",(x),(y),(frameCount),(frameDelay),(looptype)
struct Animation
{
    //sprite inheritance how
    frameCount: i32,
    frameDelay: i32,
    looptype: LoopType,
}
impl Animation
{
    fn generate(obj: &self);
}

// Sample,<time>,<layer_num>,"<filepath>",<volume>
struct Sample
{
    time: Mili,
    layer: Layer,
    filepath: String,
    volume: i32, // 0 to 100
}
impl Sample
{
    fn generate(obj: &self);
}

enum Object
{
    Sprite(Sprite),
    Animation(Animation),
}

/* COMMANDS TYPES */

enum Parameter
{
    FlipHorizontal
    {
        enabled: bool
    },
    FlipVertical
    {
        enabled: bool
    },
    AdditiveColour
    {
        enabled: bool
    },
}
impl Parameter
{
    fn value(parameter: &self) -> u8
    {
        match parameter
        {
            FlipHorizontal(val) =>
            {
                if val
                {
                    1
                }
                0
            }
            FlipVertical(val) =>
            {
                if val
                {
                    1
                }
                0
            }
            AdditiveColour(val) =>
            {
                if val
                {
                    1
                }
                0
            }
        }
    }
}

enum HitsoundsSampleSet
{
    All,
    Normal,
    Soft,
    Drum,
}
impl HitsoundsSampleSet
{
    fn value(hitsoundssampleset: &self) -> str
    {
        match hitsoundssampleset 
        {
            All => "All",
            Normal => "Normal",
            Soft => "Soft",
            Drum => "Drum",
        }
    }
}

enum HitsoundsAddition
{
    Whistle,
    Finish,
    Clap,
}
impl HitsoundsAddition
{
    fn value(hitsoundsaddition: &self) -> str
    {
        match hitsoundsaddition
        {
            Whistle => "Whistle",
            Finish => "Finish",
            Clap => "Clap",
        }
    }
}

enum TriggerType
{
    HitSound
    {
        //HitSound[SampleSet][AdditionsSampleSet][Addition][CustomSampleSet]
        sampleSet: Option<HitsoundsSampleSet>,
        additionssampleset: Option<HitsoundsSampleSet>,
        addition: Option<HitsoundsAddition>,
        customsampleset: Option<u64>,
    },
    Passing,
    Failing,
}
impl TriggerType
{
    fn value(triggertype: &self) -> String
    {
        match triggertype
        {
            HitSound(val) =>
            {
                let mut result: String;
                match val.sampleSet
                {
                    Some =>
                    {}
                    None =>
                    {}
                }
                match val.additionssampleset
                {
                    Some =>
                    {}
                    None =>
                    {}
                }
                match val.addition
                {
                    Some =>
                    {}
                    None =>
                    {}
                }
                match val.customsampleset
                {
                    Some =>
                    {}
                    None =>
                    {}
                }
            }
            Passing => "Passing",
            Failing => "Failing",
        }
    }
}

enum Easing
{
    Linear,
    EasingOut,
    EasingIn,
    QuadIn,
    QuadOut,
    QuadInOut,
    CubicIn,
    CubicOut,
    CubicInOut,
    QuartIn,
    QuartOut,
    QuartInOut,
    QuintIn,
    QuintOut,
    QuintInOut,
    SineIn,
    SineOut,
    SineInOut,
    ExpoIn,
    ExpoOut,
    ExpoInOut,
    CircIn,
    CircOut,
    CircInOut,
    ElasticIn,
    ElasticOut,
    ElasticHalfOut,
    ElasticQuarterOut,
    ElasticInOut,
    BackIn,
    BackOut,
    BackInOut,
    BounceIn,
    BounceOut,
    BounceInOut,
}
impl Easing
{
    fn value(easing: &self) -> i32
    {
        match easing
        {
            Linear => 0,
            EasingOut => 1,
            EasingIn => 2,
            QuadIn => 3,
            QuadOut => 4,
            QuadInOut => 5,
            CubicIn => 6,
            CubicOut => 7,
            CubicInOut => 8,
            QuartIn => 9,
            QuartOut => 10,
            QuartInOut => 11,
            QuintIn => 12,
            QuintOut => 13,
            QuintInOut => 14,
            SineIn => 15,
            SineOut => 16,
            SineInOut => 17,
            ExpoIn => 18,
            ExpoOut => 19,
            ExpoInOut => 20,
            CircIn => 21,
            CircOut => 22,
            CircInOut => 23,
            ElasticIn => 24,
            ElasticOut => 25,
            ElasticHalfOut => 26,
            ElasticQuarterOut => 27,
            ElasticInOut => 28,
            BackIn => 29,
            BackOut => 30,
            BackInOut => 31,
            BounceIn => 32,
            BounceOut => 33,
            BounceInOut => 34,
        }
    }
}

/* COMMANDS */

//_(event),(easing),(starttime),(endtime),(params...)
struct Commands
{
    obj: Object,
    commands: Vec<Command>,
}
struct Command_base
{
    easing: Easing,
    starttime: Mili,
    endtime: Mili,
}

enum Command
{
    Fade
    {
        startopacity: f64, // 0 to 1
        endopacity: f64,   // 0 to 1
    },
    Move
    {
        startcoordinates: Coordinates,
        endcoordinates: Coordinates,
    },
    MoveX,
    MoveY,

    Scale
    {
        startscale: f64, // def 1 -> 0 to inf
        endscale: f64,   // def 1 -> 0 to inf
    },
    VectorScale
    {
        startscaleX: f64, // def 1 -> 0 to inf
        endscaleX: f64,   // def 1 -> 0 to inf
        startscaleY: f64, // def 1 -> 0 to inf
        endscaleY: f64,   // def 1 -> 0 to inf
    },
    Rotate
    {
        startrotate: f64, // 0>
        end_rotate: f64,  // 0>
    },
    Color,
    Parameter(Parameter),
    Loop
    {
        /*
            (starttime) is the timestamp at which the loop begins
            (loopcount) is the number of times the loop executes before stopping
            (relative_starttime) is the amount of time since the start of that iteration that this event should begin
            (relative_endtime) is the amount of time since the start of that iteration that this event should end
        */
        loopcount: u32,
        commands: Vec<Command>,
    },
    Trigger
    {
        triggerType: TriggerType,
        commands: Vec<Command>,
        groupnumber: Optional<u32>,
        /*

            (triggerType) indicates the trigger condition and can be one of the following:
                HitSound[SampleSet] [AdditionsSampleSet] [Addition] [CustomSampleSet], where:
                Passing (transition from fail state to pass state)
                Failing (transition from pass state to fail state)
            (starttime) is the timestamp at which the trigger becomes valid
            (endtime) is the timestamp at which the trigger stops being valid
            (relative_starttime) is the amount of time since the trigger event that this event should begin
            (relative_endtime) is the amount of time since the trigger event that this event should end
            (group_number) (optional, default value is 0 for no group) allows triggers on the same sprite to be grouped so that all triggers of the group are stopped when one trigger starts.
        */
    },
}
impl Command::Fade {}
