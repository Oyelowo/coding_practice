// New type idiom concept from rust implemented in typescript

class Years {
  #number: number;

  constructor(num: number) {
    this.#number = num;
  }

  get number() {
    return this.#number;
  }

  to_days = (): Days => {
    return new Days(this.#number * 365);
  };
}
class Days {
  // Shorthand alternative of the Years constructor
  constructor(private number: number) {}

  to_years = (): Years => {
    return new Years(this.number / 365);
  };
}

function old_enough(age: Years): boolean {
  return age.number >= 18;
}

let age = new Years(5);
let age_days = age.to_days();
console.log(`Old enough ${old_enough(age)}`);
console.log(`Old enough ${old_enough(age_days.to_years())}`);
// console.log(`Old enough ${old_enough(age_days)}`);
