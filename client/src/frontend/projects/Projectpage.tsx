
import Navbar from '../../components/Navbar';
import Cards_page from './Cards_page';
function Projectpage() {

  return (
    <>
          <div className="bg-gray-900 min-h-screen grid grid-rows-12  gap-5 ">
              {/* navbar */}
              <div className="row-start-1 row-end-2 ">
                <Navbar />
              </div>
              {/* Main Content */}
              <div className="row-start-1 row-end-11 ">
                  <Cards_page />
              </div>
          </div>
    </>
  )
}

export default Projectpage
