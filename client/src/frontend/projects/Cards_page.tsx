
import Cards from './Cards';
import React, { useEffect, useState } from "react";

function Cards_page() {
  useEffect(() => {
    fetch('http://localhost:3000/project')
      .then(res => res.json())
      .then(getproject)
      .catch(console.error);
  }, []);
  const [projects, getproject] = useState([]);

  
  console.log(projects)
  const items = [];
  for (let project of projects) {
    items.push( <Cards
      key={project.project_link}
      name={project.name}
      img_link={project.img_link}
      project_link={project.project_link}
      text={project.text}
    />);  
  } 
    return (
      <>
        <div className="flex flex-wrap gap-4 justify-center items-center p-4">
          {items}
        </div>
      </>
    )
  }
  
  export default Cards_page
  