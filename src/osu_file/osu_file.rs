pub mod osu_file_format_v14 {

    struct Mili {} // must be a proper time management 


    /* TYPES */

    enum Countdown {
        NoCountdown,
        Normal,
        Half,
        Double,
    }
    impl Countdown
    {
        fn value(countdown: &self) -> u8
        {
            match countdown
            {
                NoCountdown => 0,
                Normal => 1,
                Half => 2,
                Double => 3,
            }
        }
    }

    enum GameMode
    {
        Osu,
        OsuTaiko,
        OsuCatch,
        OsuMania,
    }
    impl GameMode
    {
        fn value(gameMode: &self) -> u8
        {
            match gameMode
            {
                Osu => 0,
                OsuTaiko => 1,
                OsuCatch => 2,
                OsuMania => 3,
            }
        }
    }

    enum GeneralSampleSet {
        Normal, // Normal
        Soft, // Soft
        Drum, // Drum
    }
    impl GeneralSampleSet
    {
        fn value(sampleset: &self) -> str
        {
            match sampleset
            {
                Normal => "Normal",
                Soft => "Soft",
                Drum => "Drum",
            }
        }
    }

    enum OverlayPosition {
        NoChange, // NoChange
        Below, // Below
        Above, // Above
    }
    impl OverlayPosition
    {
        fn value(overlayposition: &self) -> str
        {
            match overlayposition
            {
                NoChange => "NoChange",
                Below => "Below",
                Above => "Above",
            }
        }
    }


    /* STRUCTS */


    struct General {
        audiofilename: String, // ???
        audioleadin: Mili,     // 0
        //audio_hash: String,      // ???
        previewtime: Mili,        // -1
        countdown: Countdown,      // Countdown::Normal
        sampleset: GeneralSampleSet,         // SampleSet::Normal
        stackleniency: f32,       // 0.7
        gamemode: GameMode,      // GameMode::Osu
        letterboxinbreaks: bool, // false
        countdownoffset: i32,             // beats 0
        specialstyle: bool,               // false
        widescreenstoryboard: bool,       // false
        samplesmatchplaybackrate: bool, // false
    }
    struct Editor 
    {
        bookmarks: Vec<Mili>,
        distancespacing: f32,
        beatdivisor: f32,
        gridsize: i32,
        timelinezoom: f32,
    }
    struct Metadata 
    {
        title: String,
        titleunicode: String,
        artist: String,
        artistunicode: String,
        creator: String,
        version: String,
        source: String,
        tags: Vec<String>,
        beatmapid: i32,
        beatmapsetid: i32,
    }
    struct Difficulty
    {
        hpdrainrate: f32, // (0–10)
        circlesize: f32, // (0–10)
        overalldifficulty: f32, // (0–10)
        approachrate: f32, // (0–10)
        slidermultiplier: f32,
        slidertickrate: f32,
    }


    struct Events {}


    enum TimingPointSampleSet
    {
        Default,
        Normal,
        Soft,
        Drum,
    }
    enum TimingPointEffects
    {
        KiaiEnabled, // 0
        KiaiDisabled, // 1 ~ !0 && !3 
        Barline, // 3
    }

    // time,beatLength,meter,sampleSet,sampleIndex,volume,uninherited,effects
    enum TimingPoint
    {
        Inherited // green
        {
            // gamer
            time: Mili,
            sampleSet: TimingPointSampleSet,
            sampleIndex: u32, // (0 - 99)
            volume: u32, // (0 - 99)
            effects: TimingPointEffects,
            // gamer

            slidermultiplier: f32,
        },
        Uninherited // red
        {
            beatLength: f32,
            bpm: f32, // 60000 / beatLength
            meter: i32,
        },
    }
    impl TimingPoint::Inherited
    {

    }
    impl TimingPoint::Uninherited
    {
        
    }

    struct ColoursCombo 
    {
        number: i32,
        color: Color,
    }

    struct Colours 
    {
        combo: Vec<ColoursCombo>,
        slidertrackoverride: Optional<Color>,
        sliderborder: Optional<Color>,
    }
    //x,y,time,type,hitSound,objectParams,hitSample

    enum HitObjects 
    {
        Hitcircle,
        Slider,
        Spinner,
        Maniahold,
    }
    impl


    /* STRUCT */

    struct Osu {
        general: General,
        editor: Editor,
        metadata: Metadata,
        difficulty: Difficulty,

        events: Events,
        timing_points: Vec<TimingPoint>,
        colours: Colours,
        hit_objects: HitObjects,
    }

    impl Osu {
        fn new(&mut self) -> &mut self
        {

        }
    }
}
