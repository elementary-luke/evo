for i in 0..self.tree_bodies[row_index].len()
                {
                    if i != self.tree_bodies[row_index].len() - 1
                    {
                        self.tree_bodies[row_index][i + 1].pos.x += x;
                    }
                }