use lazy_static::lazy_static;


lazy_static! {
    pub static ref CASSETTE_DISTRIBUTIONS: Vec<&'static str> = vec![
        // vanila Места, где появляются диски и шансы
        "BandPracticeInstruments", // 20 10
        "BedroomDresser", // 2
        "BedroomSideTable", // 2
        "ClosetShelfGeneric", // 2
        "CrateCompactDiscs", // 50 20 20 10 10
        "CrateRandomJunk", // 1
        "DeskGeneric", // 2
        "DresserGeneric", // 2
        "ElectronicStoreMusic", // 20 20 10 10
        "ElectronicStoreMusic", // junk 100
        "FactoryLockers", // 2
        "FireDeptLockers", // 2
        "FitnessTrainer", // 2
        "Gifts", // 2
        "GolfLockers", // 2
        "GymLockers", // 2
        "Hobbies", // 2
        "HolidayStuff", // 2
        "HospitalLockers", // 2
        "LivingRoomShelf", // 4
        "LivingRoomShelfNoTapes", // 4
        "LivingRoomSideTable", // 2
        "LivingRoomSideTableNoRemote", // 2
        "Locker", // 2
        "LockerClassy", // 2
        "MusicStoreCDs", // 50 20 20 10 10
        "MusicStoreSpeaker", // 20 20 10 10
        "OfficeDesk", // 2
        "OfficeDeskHome", // 2
        "OfficeDrawers", // 2
        "PoliceDesk", // 2
        "PoliceLockers", // 2
        "SchoolLockers", // 4
        "SecurityLockers", // 2
        "ShelfGeneric", // 2
        "WardrobeChild", // 2
        "WardrobeMan", // 2
        "WardrobeManClassy", // 2
        "WardrobeRedneck", // 2
        "WardrobeWoman", // 2
        "WardrobeWomanClassy", // 2
    ];
}
