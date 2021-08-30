import { palindrome } from '../utils/testing';
 
describe('palindrome', () => {
    test('palindrome of a', () => {
        expect(palindrome('a')).toBe('a');
      }); 

      test('palindrome of oyelowo', () => {
        expect(palindrome('oyelowo')).toBe('owoleyo');
      })

      test('palindrome of university', () => {
        expect(palindrome('university')).toBe('ytisrevinu');
      })
})


