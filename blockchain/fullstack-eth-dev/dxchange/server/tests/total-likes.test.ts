import list_helper from '../utils/list_helper';
import test_helper from './test_helper';

describe('Blogs', () => {
  test('when list has only one blog equals the likes of that', () => {
    const result = list_helper.getTotalUpvotes([test_helper.initialBlogs[0]]);
    expect(result).toBe(7);
    expect(list_helper.getTotalUpvotes([])).toBe(0)
  });

  test('returns total of all upvotes of all blogs', () => {
    const result = list_helper.getTotalUpvotes(test_helper.initialBlogs);
    expect(result).toBe(50);
  });

  test('returns blog with most upvotes', () => {
    expect(list_helper.getFavoriteBlog(test_helper.initialBlogs)).toEqual({
      title: 'Canonical string reduction',
      author: 'Edsger W. Dijkstra',
      url: 'http://www.cs.utexas.edu/~EWD/transcriptions/EWD08xx/EWD808.html',
      upvotes: 12
    });

    expect(list_helper.getFavoriteBlog([])).toEqual({});
  });


  test('return Author With Most Blogs', () => {
    expect(list_helper.getAuthorWithMostBlogs(test_helper.initialBlogs)).toEqual({
      author: 'Edsger W. Dijkstra',
      blogs: 4 
    })
  })
  
  test('return Author With Most Liked Blog post', () => {
    expect(list_helper.getAuthorWithMostLikedBlog(test_helper.initialBlogs)).toEqual({
      author: 'Edsger W. Dijkstra',
      upvotes: 12 
    }) 
  })
});
