'use client'
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import Link from 'next/link';

export default function Home() {

  const handleGetStartedTest = () => {
    console.log("Getting Started was pressed.");
  }

  return (
    <div className="grid grid-rows-[20px_1fr_20px] items-center justify-items-center min-h-screen p-8 pb-20 gap-16 sm:p-20 font-[family-name:var(--font-geist-sans)]">
      <main className="flex flex-col gap-8 row-start-2 items-center sm:items-start">
        <div>
          <h1>FinTrack</h1>
        </div>
        <Card>
          <CardHeader>
            <CardTitle>Sign Up</CardTitle>
            <CardDescription>First steps to a wealthier you.</CardDescription>
          </CardHeader>
          <CardContent>
            <form>
              <div className='grid w-full items-center gap-4'>
                <div className='flex flex-row space-x-1.5'>
                  <Input id='first-name' placeholder='Ray' />
                  <Input id='last-name' placeholder='Dalio' />
                </div>
                <div className='flex flex-col space-y-1.5'>
                  <Input id='password' placeholder='password123 :)' />
                </div>
              </div>
            </form>
          </CardContent>
          <CardFooter>
            <div className='flex flex-row justify-end w-full'>
              <Button><Link href={"/dashboard"}>See Dashboard</Link></Button>
            </div>
          </CardFooter>
        </Card>
      </main>
      <footer className="row-start-3 flex gap-6 flex-wrap items-center justify-center">
        <h3>The Footer</h3>
      </footer>
    </div>
  );
}
