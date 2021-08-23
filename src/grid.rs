use super::*;

// ==========
// Grid utils
// ==========

pub fn update_grid(
    agents: Query<(&Position, &Status), With<Agent>>,
    mut grid: ResMut<Grid>,
    grid_size: Res<InputSize>,
) {
    // init empty grid
    let mut new_grid: Grid = Grid(
        (0..grid_size.0)
            .map(|_| (0..grid_size.0).map(|_| None).collect())
            .collect(),
    );
    // populate grid with agents
    agents.for_each(|(position, status)| {
        new_grid.0[position.1][position.0] = Some(status.clone());
    });
    grid.0 = new_grid.0;
}

pub fn print_grid(grid: Res<Grid>, grid_size: Res<InputSize>, tag_count: Res<TagCount>) {
    // clear terminal
    print!("\x1B[2J");
    println!(" __{}__", "__".repeat(grid_size.0));
    println!("|  {}  |", "__".repeat(grid_size.0));

    grid.0.iter().for_each(|row| {
        let line: String = row
            .iter()
            .map(|field| match field {
                Some(Status::Tagged) => "😈️",
                Some(Status::UnTaggable) => "😀️",
                Some(Status::Normal) => "🐏️",
                None => "  ",
            })
            .collect();
        println!("| |{}| |", line)
    });

    println!("| |{}| |", "__".repeat(grid_size.0));
    println!("|__{}__|", "__".repeat(grid_size.0));
    println!("Total count of exchanged tags: {}", tag_count.0)
}
