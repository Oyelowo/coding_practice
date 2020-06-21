const factorial = (n: number) => {
  if (n === 0) return 1;
  return factorial(n - 1) * n;
};

//console.log(factorial(60));

const fib = (n: number) => {
  if (n <= 2) return 1;
  return fib(n - 1) + fib(n - 2);
};

//console.log(fib(12));

// Bottom up approach is a lot more efficient with 0(n) time complexity
const fib_buttom_up = (n: number) => {
  if (n < 3) return 1;
  let arr = [null, 1, 1];
  for (let i = 3; i <= n; i++) {
    arr[i] = arr[i - 1] + arr[i - 2];
  }
  return arr[n];
};

console.log(fib_buttom_up(669));

const camelize = (word: string) =>
  word
    .split("_")
    .map((part, i) =>
      i === 0 ? part.toLowerCase() : `${part[0].toUpperCase()}${part.slice(1)}`
    )
    .join("");

const isObj = (obj: object) => obj !== null && obj.constructor === Object;

const converKeysShallow = (obj: object) => {
  const newObj = {};
  Object.keys(obj).forEach((prop) => {
    newObj[camelize(prop)] = obj[prop];
  });
  return newObj;
};

/* const convertKeys = (obj: object) => {
  for (const k in obj) {
    if (typeof obj[k] == "object" && obj[k] !== null) {
      const newObj = {};
      Object.keys(obj).forEach((prop) => {
        newObj[camelize(prop)] = obj[prop];
      });
    } else {
      return converKeysShallow(obj);
    }
    // do something...
  }
}; */

const convertKeys = (obj: object) => {
  if (isObj(obj)) {
    return Object.keys(obj).reduce((acc, val) => {
      return {
        ...acc,
        [camelize(val)]: convertKeys(obj[val]),
      };
    }, {});
  }
  return obj;
};

const someData = {
  age_as_at_now: 45,
  early_data: ["here", "and", "there"],
  subject_taken: ["phy", "maths"],
  course_mates: {
    sonja_koivi: {
      age_as_at_now: 99,
      class_taken: 344,
      info_from: {
        at: "434",
        created_at: "k",
        updated_at: {
          place_of_update: "Finland",
          time_of_update: "232332",
        },
      },
    },
    sam_sake: {
      age_as_at_now: 99,
      class_taken: 542,
    },
    oyelowo_oyedayo: {
      age_as_at_now: 99,
      class_taken: 1044,
    },
    kaks_ppo: {},
  },
};

const kk = convertKeys(someData);
console.log(JSON.stringify(kk));

/* const camelizeKeys = (obj) => {
  if (Array.isArray(obj)) {
    return obj.map((v) => camelizeKeys(v));
  } else if (obj !== null && obj.constructor === Object) {
    return Object.keys(obj).reduce(
      (result, key) => ({
        ...result,
        [camelize(key)]: camelizeKeys(obj[key]),
      }),
      {}
    );
  }
  return obj;
}; */



const camelizeKeys = (obj: object) => {
  if (isObj(obj)) {
    return Object.keys(obj).reduce(
      (result, key) => ({
        ...result,
        [camelize(key)]: camelizeKeys(obj[key]),
      }),
      {}
    );
  }
  return obj;
};

//console.log(camelizeKeys(someData));
