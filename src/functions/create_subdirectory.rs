pub mod create_subdirectory {
    use crate::functions::check_directory::check_directory::check_directory;
    pub async fn create_subdirectory(new_clean_directory: &str) {
        println! {"Subdirectory: {}", new_clean_directory};

        let electric_directory: String = new_clean_directory.to_owned() + "/ElectricShuffle";
        let red_engine_directory: String = new_clean_directory.to_owned() + "/RedEngine";
        let flight_club_directory: String = new_clean_directory.to_owned() + "/FlightClub";

        check_directory(&electric_directory).await;
        check_directory(&red_engine_directory).await;
        check_directory(&flight_club_directory).await;
    }
}
