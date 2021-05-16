import React, { FC } from "react";

const If: FC<{ condition: boolean }> = ({ children, condition }) => {
  return <div>{condition && children}</div>;
};

export default If;
