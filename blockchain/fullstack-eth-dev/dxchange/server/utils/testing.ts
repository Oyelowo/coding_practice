export const palindrome = (string: string) =>
  string
    .split('')
    .reverse()
    .join('');

export const average = (array: number[]) => {
  const reducer = (acc: number, val: number): number => acc + val;
  return array.length === 0 ? 0 : array.reduce(reducer, 0) / array.length;
};
