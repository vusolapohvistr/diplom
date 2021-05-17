use std::usize;

use rand::{Rng, prelude::SliceRandom, thread_rng};

use crate::lib::{distance::Distance, tsp_algs::tsp_alg::TSPSolver};

pub struct GeneticAlgTSPSolver {
  population_size: usize,
  gens_count: usize,
  crossover_rate: f64,
  mut_rate: f64,
  dying_rate: f64,
}

impl GeneticAlgTSPSolver {
  pub fn new(
    population_size: usize,
    gens_count: usize,
    crossover_rate: f64,
    mut_rate: f64,
    dying_rate: f64,
  ) -> Self {
    GeneticAlgTSPSolver {
      population_size,
      gens_count,
      crossover_rate,
      mut_rate,
      dying_rate,
    }
  }

  fn sort_population<T: Distance + Clone>(
    &self,
    points: &Vec<T>,
    distances_mat: &Vec<Vec<f64>>,
    population: &mut Vec<Vec<usize>>
  ) {
    population
      .sort_by(|a, b| 
        calc_fitness(&points, &distances_mat, &a)
          .partial_cmp(&calc_fitness(&points, &distances_mat, &b))
          .unwrap()
      );
  }

  fn mutate_population(&self, population: &mut Vec<Vec<usize>>) {
    for individual in population.iter_mut() {
      self.mutate(individual);
    }
  }

  // we use swap mutation
  fn mutate(&self, individual: &mut Vec<usize>) {
    let mut rng = thread_rng();

    for swapped in 0..individual.len() {
      if rng.gen::<f64>() < self.mut_rate {
        let swap_with = rng.gen_range(0..individual.len());
        individual.swap(swapped, swap_with);
      }
    }
  }

  
  fn breed_population(&self, population: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut rng = thread_rng();

    // shuffle population to get random pairs
    let mut shuffled_pop = population.clone();
    shuffled_pop.shuffle(&mut rng);

    let mut result = Vec::with_capacity(population.len());

    shuffled_pop.chunks(2)
      .map(|parents| {
        if rng.gen::<f64>() < self.crossover_rate {
          return breed(&parents[0], &parents[1]);
        }
        (parents[0].clone(), parents[1].clone())
      })
      .for_each(|(offspring1, offspring2)| {
        result.push(offspring1);
        result.push(offspring2);
      });

    result
  }
}

impl TSPSolver for GeneticAlgTSPSolver {
  fn solve_tsp<T: Distance + Clone>(&self, points: &Vec<T>, start_point_idx: usize) -> Vec<T> {
    let distances_mat: Vec<Vec<f64>> = (0..points.len())
      .map(|a| 
        (0..points.len())
        .map(move |b| points[a].distance_to(&points[b]))
        .collect()
      )
      .collect();

    // initializing first population
    let mut population: Vec<Vec<usize>> = generate_population(&points, self.population_size);

    for _ in 0..self.gens_count {
      population = self.breed_population(&population);
      self.mutate_population(&mut population);

      // weak die
      let should_die_count = (self.population_size as f64 * self.dying_rate) as usize;

      self.sort_population(&points, &distances_mat, &mut population);
      let mut should_alive: Vec<Vec<usize>> = population.iter()
        .take(self.population_size - should_die_count)
        .cloned()
        .collect();
      // newbies arrive
      population = generate_population(&points, should_die_count);
      population.append(&mut should_alive);
    }

    let mut result = population
      .into_iter()
      .max_by(|a, b| 
        calc_fitness(&points, &distances_mat, &a)
          .partial_cmp(
            &calc_fitness(&points, &distances_mat, &b)
          ).unwrap())
      .unwrap();

    
    let offset = result.iter().position(|bit| *bit == start_point_idx).unwrap();

    result.rotate_left(offset);

    result.iter()
      .map(|point_index| points[*point_index].clone())
      .collect()
  }
}

fn generate_population<T: Distance + Clone>(points: &Vec<T>, size: usize) -> Vec<Vec<usize>> {
  let mut rng = thread_rng();

  (0..size)
    .map(|_| {
      let mut path: Vec<usize> = (0..points.len()).map(|i| i).collect();
      path.shuffle(&mut rng);

      path
    })
    .collect()
}

// higher is better
fn calc_fitness<T: Distance + Clone>(points: &Vec<T>, distances_mat: &Vec<Vec<f64>>, path: &Vec<usize>) -> f64 {
  let mut result = distances_mat[points.len() - 1][0];

  for i in 0..path.len() - 1 {
    result += distances_mat[path[i]][path[i + 1]];
  }

  // println!("{} {:?}", result, path);

  1.0 / result
}

// breed algorithm published on https://www.hindawi.com/journals/cin/2017/7430125/#B9
fn breed(parent1: &Vec<usize>, parent2: &Vec<usize>) -> (Vec<usize>, Vec<usize>) {
  let mut offspring1 = Vec::with_capacity(parent1.len());
  let mut offspring2 = Vec::with_capacity(parent1.len());

  let mut bit_index_from_step_4 = 0;

  // step 2
  offspring1.push(parent2[0]);

  loop {
    // step 3
    let bit_index_in_parent1 = parent1.iter().position(|bit| *bit == parent2[bit_index_from_step_4]).unwrap();
    let bit_index_from_step_3 = parent1.iter().position(|bit| *bit == parent2[bit_index_in_parent1]).unwrap();
    offspring2.push(parent2[bit_index_from_step_3]);

    // step 5
    if parent2[bit_index_from_step_3] == parent1[0] {
      break;
    }

    // step 4
    bit_index_from_step_4 = parent1.iter().position(|bit| *bit == parent2[bit_index_from_step_3]).unwrap();
    offspring1.push(parent2[bit_index_from_step_4]);
  }

  // step X not mentioned in publication for cases like parent1 = [2, 1, 0] and parent2 = [1, 0, 2]
  let mut forgotted_bits_1: Vec<usize> = offspring1.iter().filter(|bit| !offspring2.contains(bit)).copied().collect();
  let mut forgotted_bits_2: Vec<usize> = offspring2.iter().filter(|bit| !offspring1.contains(bit)).copied().collect();
  offspring1.append(&mut forgotted_bits_2);
  offspring2.append(&mut forgotted_bits_1);

  // step 6
  if offspring1.len() != parent1.len() {
    let to_breed_1: Vec<usize> = parent1.iter()
      .filter(|bit| !offspring2.contains(*bit))
      .copied()
      .collect();
    let to_breed_2: Vec<usize> = parent2.iter()
      .filter(|bit| !offspring1.contains(*bit))
      .copied()
      .collect();
    
    let (mut offspring1_tail, mut offspring2_tail) = breed(&to_breed_1, &to_breed_2);
    offspring1.append(&mut offspring1_tail);
    offspring2.append(&mut offspring2_tail);
  }

  (offspring1, offspring2)
}

#[test]
fn breed_test_1() {
  let parent1 = vec![3, 4, 8, 2, 7, 1, 6, 5];
  let parent2 = vec![4, 2, 5, 1, 6, 8, 3, 7];

  let offspring1_expected = vec![4, 8, 6, 2, 5, 3, 1, 7];
  let offspring2_expected = vec![1, 7, 4, 8, 6, 2, 5, 3];

  let (offspring1, offspring2) = breed(&parent1, &parent2);

  assert_eq!(offspring1, offspring1_expected);
  assert_eq!(offspring2, offspring2_expected);
}

#[test]
fn breed_test_2() {
  let parent1 = vec![1, 2, 3, 4, 5, 6, 7, 8];
  let parent2 = vec![2, 7, 5, 8, 4, 1, 6, 3];

  let offspring1_expected = vec![2, 1, 6, 7, 5, 3, 8, 4];
  let offspring2_expected = vec![6, 7, 2, 1, 8, 4, 5, 3];

  let (offspring1, offspring2) = breed(&parent1, &parent2);

  assert_eq!(offspring1, offspring1_expected);
  assert_eq!(offspring2, offspring2_expected);
}

#[test]
fn breed_test_3() {
  let parent1 = vec![2, 1, 0];
  let parent2 = vec![1, 0, 2];

  let offspring1_expected = vec![1, 2, 0];
  let offspring2_expected = vec![2, 1, 0];

  let (offspring1, offspring2) = breed(&parent1, &parent2);

  assert_eq!(offspring1, offspring1_expected);
  assert_eq!(offspring2, offspring2_expected);
}
