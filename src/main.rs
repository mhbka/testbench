// Draw and Phase are now typestates of Phase
trait Phase {}

struct Draw;
struct Play;

impl Phase for Draw {}
impl Phase for Play {}

// testing
trait GameFamily {
    type Game<P: Phase>;
}

trait GameNext<F: GameFamily> {
    type 
}

struct Basic<P: Phase> {
    phase: P
}

impl<P: Phase> GameFamily for Basic<P> {
    type Game<Ph: Phase> = Basic<P>;
}

fn main() {

}
