
import Cards from '../../components/Cards';
import React, { useEffect, useState } from "react";

function Cards_page() {
  useEffect(() => {
    fetch('http://localhost:3000/project')
      .then(res => res.json())
      .then(getproject)
      .catch(console.error);
  }, []);
  const [projects, getproject] = useState([]);


    return (
      <>
        <div className="flex flex-wrap gap-4 justify-center items-center p-4">

        </div>
      </>
    )
  }
  
  export default Cards_page
  