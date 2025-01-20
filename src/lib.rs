use bevy_life::WireWorldCellState::{Conductor, ElectronHead, ElectronTail};

pub struct State {
    pub cells: ndarray::Array<Option<bevy_life::WireWorldCellState>, ndarray::Ix2>,
}

impl<I: ca_formats::Input> From<ca_formats::rle::Rle<I>> for State {
    fn from(rle: ca_formats::rle::Rle<I>) -> Self {
        let width: usize;
        let height: usize;
        let mut x_start = 0;
        let mut y_start = 0;

        match rle.header_data() {
            Some(ca_formats::rle::HeaderData { x, y, .. }) => {
                width = *x as usize;
                height = *y as usize;
            }
            None => panic!("Could not read RLE header data."),
        }

        match rle.cxrle_data() {
            Some(ca_formats::rle::CxrleData {
                pos: Some((x, y)), ..
            }) => {
                x_start = x.clone();
                y_start = y.clone();
            }
            None => (),
            data => panic!("Could not read RLE context data: {:?}", data),
        }

        let mut cells = ndarray::Array::from_elem((height, width), None);

        for cell in rle {
            match cell {
                Ok(ca_formats::CellData {
                    position: (x, y),
                    state,
                }) => {
                    let x: usize = (x - x_start).try_into().unwrap();
                    let y: usize = (y - y_start).try_into().unwrap();

                    cells[(y, x)] = match state {
                        1 => Some(ElectronHead),
                        2 => Some(ElectronTail),
                        3 => Some(Conductor),
                        _ => None,
                    }
                }
                Err(error) => panic!("{}", error),
            };
        }

        State { cells: cells }
    }
}

#[cfg(test)]
mod tests {
    use crate::State;
    use bevy_life::WireWorldCellState::{Conductor, ElectronHead, ElectronTail};

    #[test]
    fn test_from_rle() {
        let rle =
            ca_formats::rle::Rle::new("x = 4, y = 3, rule = WireWorld\n.BA$C2.C$.2C!").unwrap();

        let state = State::from(rle);

        assert_eq!(state.cells.shape(), [3, 4]);

        assert_eq!(state.cells[(0, 0)], None);
        assert_eq!(state.cells[(0, 1)], Some(ElectronTail));
        assert_eq!(state.cells[(0, 2)], Some(ElectronHead));
        assert_eq!(state.cells[(1, 0)], Some(Conductor));
    }

    #[test]
    fn test_from_rle_with_negative_indexes() {
        let rle = ca_formats::rle::Rle::new(
            "#CXRLE Pos=-1,-1\nx = 4, y = 3, rule = WireWorld\n.BA$C2.C$.2C!",
        )
        .unwrap();

        let state = State::from(rle);

        assert_eq!(state.cells.shape(), [3, 4]);

        assert_eq!(state.cells[(0, 0)], None);
        assert_eq!(state.cells[(0, 1)], Some(ElectronTail));
        assert_eq!(state.cells[(0, 2)], Some(ElectronHead));
        assert_eq!(state.cells[(1, 0)], Some(Conductor));
    }
}
