import Head from "next/head";
import styles from "../styles/Home.module.css";
import useForm from "./Hooks/Input";


export default function Home() {
  const { getInputAttributes: getInputAttrs } = useForm(
    { age: 99, name: "Oyelowo", password: "" },
    (inp) => ({
      age: inp.age < 4 && "Tough life",
      name: inp.name.length < 3 && "Hmmm, Mayber longer",
      password: inp.password.length < 5 && "Get stronger. Will you?",
    })
  );
  const ageInput = getInputAttrs("age");
  const nameInput = getInputAttrs("name");
  const passwordInput = getInputAttrs("password");

  console.log(ageInput.value);

  return (
    <div className={styles.container}>
      <Head>
        <title>Create Next App</title>
        <link rel="icon" href="/favicon.ico" />
      </Head>
      {ageInput.error}
      <input {...ageInput} type="number" />
      {nameInput.error}
      <input {...nameInput} type="text" />

      {passwordInput.error}
      <input {...passwordInput} type="text" />
    </div>
  );
}
