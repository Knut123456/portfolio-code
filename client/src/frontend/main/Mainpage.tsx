
import Navbar from '../../components/Navbar';
import Profileinfo from "../../components/Profileinfo"; 


function Mainpage() {

  return (
    <>
          <div className="bg-gray-900  min-h-screen grid grid-rows-10  gap-5 ">
              {/* navbar */}
              <div className="row-start-1 row-end-2 ">
                <Navbar />
              </div>
              {/* Main Content */}
              <div className="row-start-2 row-end-11 ">
                <Profileinfo />
              </div>
          </div>
    </>
  )
}

export default Mainpage
