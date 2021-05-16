import { Button } from "@chakra-ui/react";
import Link from "next/link";

const Header = () => {
  return (
    <div
      style={{
        display: "flex",
        justifyContent: "space-between",
        maxWidth: "70vw",
      }}
    >
      <Link href="/">
        <a>CrowdSource Coin</a>
      </Link>
      <Link href="/">
        <a>campaigns</a>
      </Link>
      <Link href="/campaigns/new">
        <Button>
          <a>+</a>
        </Button>
      </Link>
    </div>
  );
};

export default Header;
