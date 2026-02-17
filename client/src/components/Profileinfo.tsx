import profile_picture from "/profile_picture.jpg" 
function Profileinfo() {
    return(
        <>
            <div className=" w-full flex flex-col md:flex-row justify-center items-center bg-gray-900 shadow-md rounded-lg p-4  gap-10">
                <div className="w-1/2">
                    <img src={profile_picture} className=" w-4/6 h-4/6 rounded-full" alt="Knut Røte"/>
                </div>
                <div className="flex flex-col justify-center items-start ml-4 w-1/2">
                    <div>
                        <h1 className="text-3xl text-white font-light w-1/2">Knut Røte</h1>
                        <p className="text-gray-500 w-1/2">Student at Kuben IKT</p>
                    </div>
                    <p className="text-white">I am a student working up to be a software </p>
                </div>
            </div>
        </>
    )
}
export default Profileinfo