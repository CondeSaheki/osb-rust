pub mod osu_file_format_v14 {

    struct Mili {} // must be a proper time management 


    enum Countdown {
        NoCountdown, // 0
        Normal,       // 1
        Half,         // 2
        Double,       // 3
    }
    enum GameMode
    {
        Osu,       // 0
        OsuTaiko, // 1
        OsuCatch, // 2
        OsuMania, // 3
    }
    enum SampleSet {
        Normal, // Normal
        Soft, // Soft
        Drum, // Drum
    }
    enum OverlayPosition {
        NoChange, // NoChange
        Below, // Below
        Above, // Above
    }
    struct General {
        audiofilename: String, // ???
        audioleadin: Mili,     // 0
        //audio_hash: String,      // ???
        previewtime: Mili,        // -1
        countdown: Countdown,      // Countdown::Normal
        sampleset: String,         // SampleSet::Normal
        stackleniency: f32,       // 0.7
        gamemode: GameMode,      // GameMode::Osu
        letterboxinbreaks: bool, // false1lse
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


    enum SampleSet
    {
        Default,
        Normal,
        Soft,
        Drum,
    }
    enum Effects
    {
        KiaiEnabled, // 0
        KiaiDisabled, // 1 ~ !0 && !3 
        Barline, // 3
    }
    struct TimingPointbase
    {
        time: Mili,
        sampleSet: SampleSet,
        sampleIndex: u32, // (0 - 99)
        volume: u32, // (0 - 99)
        effects: Effects,
    }
    struct TimingPointsInherited // green
    {
        timingpoint: TimingPointbase,
        slidermultiplier: f32,
    }
    struct TimingPointsUninherited // red
    {
        timingpoint: TimingPointbase,
        beatLength: f32,
        bpm: f32, // 60000 / beatLength
        meter: i32,
    }

    struct Colours {}
    struct HitObjects {}

    struct Osu {
        general: General,
        editor: Editor,
        metadata: Metadata,
        difficulty: Difficulty,

        events: Events,
        timing_points: Vec<TimingPoints>,
        colours: Colours,
        hit_objects: HitObjects,
    }

    impl Osu {
        fn new(&mut self) -> &mut self
        {

        }
    }
}
