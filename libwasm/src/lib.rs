use wasm_bindgen::prelude::*;

//🦀
//⚙

#[repr(u8)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    pub width: u16,
    pub height: u16,
    cells: Vec<Vec<Cell>>,
}

#[wasm_bindgen]
impl Universe {
    pub fn init(h: u16, w: u16, field: &[u8]) -> Universe {
        return Universe {
            height: h,
            width: w,
            cells: {
                let mut ln: Vec<Vec<Cell>> = Vec::new();
                for _h in 0..h {
                    let mut l: Vec<Cell> = Vec::new();
                    for _w in 0..w {
                        l.push(match field[(_h * w + _w) as usize] {
                            1 => Cell::Alive,
                            0 | _ => Cell::Dead,
                        })
                    }
                    ln.push(l);
                }
                ln
            },
        };
    }
    pub fn tick(&mut self) {
        let net = self.build_alive_net();
        for _h in 0..self.height as usize {
            for _w in 0..self.width as usize {
                self.cells[_h][_w] = match (self.cells.get(_h).unwrap().get(_w).unwrap(),
                                            net.get(_h).unwrap().get(_w).unwrap()) {
                    (Cell::Dead, 3) => Cell::Alive,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (_, _) => Cell::Dead
                }
            }
        }
    }
    pub fn get(&self) -> String {
        let mut st = String::new();
        for _h in 0..self.height as usize {
            for _w in 0..self.width as usize {
                st += format!("{}", match self.cells[_h][_w] {
                    Cell::Alive => 1,
                    Cell::Dead => 0
                }).as_str();
            }
        }
        return st;
    }
}

impl Universe {
    fn build_alive_net(&self) -> Vec<Vec<u8>> {
        let mut res: Vec<Vec<u8>> = Vec::new();
        for _h in 0..self.height {
            let mut r: Vec<u8> = Vec::new();
            for _w in 0..self.width {
                r.push(
                    (match self
                        .cells
                        .get(_h as usize)
                        .unwrap()
                        .get(if _w == 0 { (self.width - 1) as usize } else { (_w - 1) as usize })
                        .unwrap()
                    {
                        Cell::Alive => 1,
                        Cell::Dead => 0,
                    } as u8) +  //left
                        (match self
                            .cells
                            .get(if _h == 0 { (self.height - 1) as usize } else { (_h - 1) as usize })
                            .unwrap()
                            .get(if _w == 0 { (self.width - 1) as usize } else { (_w - 1) as usize })
                            .unwrap()
                        {
                            Cell::Alive => 1,
                            Cell::Dead => 0,
                        } as u8) + //up-left
                        (match self
                            .cells
                            .get(if _h == 0 { (self.height - 1) as usize } else { (_h - 1) as usize })
                            .unwrap()
                            .get(_w as usize)
                            .unwrap()
                        {
                            Cell::Alive => 1,
                            Cell::Dead => 0,
                        } as u8) + //up
                        (match self
                            .cells
                            .get(if _h == 0 { (self.height - 1) as usize } else { (_h - 1) as usize })
                            .unwrap()
                            .get(if _w == self.width - 1 { 0 as usize } else { (_w + 1) as usize })
                            .unwrap()
                        {
                            Cell::Alive => 1,
                            Cell::Dead => 0,
                        } as u8) + //up-right
                        (match self
                            .cells
                            .get(_h as usize)
                            .unwrap()
                            .get(if _w == self.width - 1 { 0 as usize } else { (_w + 1) as usize })
                            .unwrap()
                        {
                            Cell::Alive => 1,
                            Cell::Dead => 0,
                        } as u8) + //right
                        (match self
                            .cells
                            .get(if _h == self.height - 1 { 0 as usize } else { (_h + 1) as usize })
                            .unwrap()
                            .get(if _w == self.width - 1 { 0 as usize } else { (_w + 1) as usize })
                            .unwrap()
                        {
                            Cell::Alive => 1,
                            Cell::Dead => 0,
                        } as u8) + //down-right
                        (match self
                            .cells
                            .get(if _h == self.height - 1 { 0 as usize } else { (_h + 1) as usize })
                            .unwrap()
                            .get(_w as usize)
                            .unwrap()
                        {
                            Cell::Alive => 1,
                            Cell::Dead => 0,
                        } as u8) + //down
                        (match self
                            .cells
                            .get(if _h == self.height - 1 { 0 as usize } else { (_h + 1) as usize })
                            .unwrap()
                            .get(if _w == 0 { (self.width - 1) as usize } else { (_w - 1) as usize })
                            .unwrap()
                        {
                            Cell::Alive => 1,
                            Cell::Dead => 0,
                        } as u8) //down-left
                );
            }
            res.push(r);
        }
        return res;
    }
}
