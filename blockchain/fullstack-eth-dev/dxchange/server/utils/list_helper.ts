import { IBlog, IBlogDocument } from '../models/blog';
import _ from 'lodash';

const dummy = (blogs: IBlog[]) => {
  return 1;
};

//export type IBlog = IBlog & { _id: string; __v: number };

type KeysOfInterest = '_id' | 'title' | 'author' | 'url' | 'upvotes' | '__v';
type IBlog2 = Pick<IBlogDocument, KeysOfInterest>;
const getTotalUpvotes = (blogs: IBlog[]) => {
  return blogs.reduce((acc, { upvotes }) => acc + upvotes, 0);
};

interface FavoriteBlog {
  title: string;
  author: string;
  url: string;
  upvotes: number;
}

const getFavoriteBlog = (blogs: IBlog[]): FavoriteBlog | {} => {
  if (blogs.length === 0) return {};
  const { title, author, upvotes, url } = blogs.sort((a, b) => b.upvotes - a.upvotes)[0];
  return { title, author, upvotes, url };
};

const groupBy = <T, K extends keyof T>(arrayOfobj: T[], key: K): { [key: string]: number } =>
  arrayOfobj.reduce((acc: any, val: any) => {
    let value = val[key];
    return {
      ...acc,
      [value]: acc[value] ? (acc[value] += 1) : 1
    };
  }, {});

interface PopularAuthor {
  author: string;
  blogs: number;
}
// First Solution: Wheel reivented here(groupBy fn above) just for practise
const getAuthorWithMostBlogs = (blogs: IBlog[]): PopularAuthor => {
  const groupedAuthors = groupBy(blogs, 'author');
  return Object.entries(groupedAuthors)
    .map(([author, blogs]) => ({ author, blogs }))
    .sort((a, b) => b.blogs - a.blogs)[0];
};

/*  SECOND SOLUTION 
   const groupedAuthors = _.groupBy(blogs, 'author');
  const authorsToKeyValueArray = Object.entries(groupedAuthors);
  return authorsToKeyValueArray
    .map(([author, blogs]) => ({ author, blogs: blogs.length }))
    .sort((a, b) => b.blogs - a.blogs)[0]; */

/*   // Third Solution
  const sep = _.groupBy(_.groupBy(blogs, 'author'), 'length');
  const maxBlogs = Math.max(...Object.keys(sep).map(el => Number(el)));
  return {
    author: sep[maxBlogs][0][0].author,
    blogs: maxBlogs
  }; */

const getAuthorWithMostLikedBlog = (blogs: IBlog[]) => {
  const { author, upvotes } = blogs.sort((a, b) => b.upvotes - a.upvotes)[0];
  return { author, upvotes };
};

export default {
  dummy,
  getTotalUpvotes,
  getFavoriteBlog,
  getAuthorWithMostBlogs,
  getAuthorWithMostLikedBlog
};
