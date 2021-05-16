import React, { FC } from "react";
import Header from "./Header";

const Layout: FC = ({ children }) => {
  return (
    <div>
      <Header />
      <hr />
      <hr />
      <hr />
      <br />

      {children}

      <br />
      <br />
      <hr />
      <hr />
      <hr />
      <h1>The footer</h1>
    </div>
  );
};

export default Layout;
