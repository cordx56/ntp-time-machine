import Head from "next/head";

import SetTimeForm from "@/components/SetTimeForm";

const Index = () => {
  return (
    <>
      <Head>
        <title>ntp-time-machine - Set your NTP client to desired time</title>
      </Head>
      <header>
        <h1 className="text-2xl">ntp-time-machine</h1>
      </header>
      <main>
        <SetTimeForm />
      </main>
      <footer>prod by cordx56</footer>
    </>
  );
};

export default Index;
