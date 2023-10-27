import { useDispatch, useSelector } from 'react-redux';
import { useNavigate } from 'react-router-dom';
import * as Yup from 'yup';
import { Field, Form, Formik } from 'formik';
import { Room } from 'rust-tc-sdk';
import { roomsSelector, setRoom } from '../redux/room';
import icon from '../../../assets/icon.svg';

const roomSchema = Yup.object().shape({
  id: Yup.string()
    .required('ID is required')
    .default(() => Math.random().toString(36)),
  name: Yup.string().required('Name is required'),
  secretToken: Yup.string()
    .required('Secret is required')
    .default(() => Math.random().toString(36)),
});

export function EnterRoomPage() {
  const navigate = useNavigate();
  const rooms = useSelector(roomsSelector);
  const dispatch = useDispatch();

  const onSubmit = (value: Partial<Room>) => {
    console.log({ value });
    dispatch(setRoom(value as Room));
    navigate(`/room/${value.id}`);
  };

  return (
    <div className="w-full max-w-md px-4 mx-auto py-16">
      <img className="pb-4 mx-auto" width="144" alt="icon" src={icon} />
      <h1 className="text-4xl pb-4 font-bold text-center">Enter Room</h1>
      <Formik<Partial<Room>>
        validationSchema={roomSchema}
        initialValues={{ name: '' }}
        onSubmit={onSubmit}>
        {({ isSubmitting }) => (
          <Form className="pb-2">
            <Field name="name">
              {({ field, meta }: any) => (
                <div className="mb-4">
                  <input
                    type="text"
                    {...field}
                    className="input input-bordered w-full"
                    placeholder="Enter Room Name"
                  />
                  {meta.touched && meta.error && (
                    <div className="error">{meta.error}</div>
                  )}
                </div>
              )}
            </Field>

            <button
              type="submit"
              disabled={isSubmitting}
              className="btn btn-block">
              Join Room
            </button>
          </Form>
        )}
      </Formik>
      <div className="divider" />
      <div>
        {rooms.map((room) => (
          <button
            type="button"
            onClick={() => navigate(`/room/${room.id}`)}
            key={room.id}
            className="kbd kbd-md mr-2 mb-2">
            {room.name}
          </button>
        ))}
      </div>
    </div>
  );
}
