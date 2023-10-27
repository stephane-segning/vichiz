import { useDispatch, useSelector } from 'react-redux';
import { useNavigate } from 'react-router-dom';
import * as Yup from 'yup';
import { Field, Form, Formik } from 'formik';
import type { Room, RoomOption } from 'rust-tc-sdk';
import React, { useCallback, useMemo } from 'react';
import { unwrapResult } from '@reduxjs/toolkit';
import { X } from 'react-feather';
import { createRoom, removeRoom, roomsSelector } from '../redux/room';
import icon from '../../../assets/icon.svg';

const roomSchema = Yup.object().shape({
  name: Yup.string().required('Name is required'),
});

export function EnterRoomPage() {
  const navigate = useNavigate();
  const roomsMap = useSelector(roomsSelector);
  const rooms = useMemo(() => Object.values(roomsMap), [roomsMap]);
  const dispatch = useDispatch();

  const onSubmit = useCallback(
    async (option: RoomOption) => {
      const resultAction = await dispatch(createRoom(option) as any);
      const room: Room = unwrapResult(resultAction);
      navigate(`/room/${room.id}`);
    },
    [dispatch, navigate],
  );

  const remove = useCallback(
    (e: React.MouseEvent<HTMLButtonElement, MouseEvent>, id: string) => {
      e.stopPropagation();
      dispatch(removeRoom(id) as any);
    },
    [dispatch],
  );

  return (
    <div className="w-full max-w-md px-4 mx-auto py-16">
      <img className="pb-4 mx-auto" width="144" alt="icon" src={icon} />
      <h1 className="text-4xl pb-4 font-bold text-center">Enter Room</h1>
      <Formik<RoomOption>
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
      {rooms.length > 0 && <div className="divider" />}
      <div>
        {rooms.map((room) => (
          <div
            onClick={() => navigate(`/room/${room.id}`)}
            key={room.id}
            className="kbd kbd-md mr-2 mb-2 cursor-pointer">
            <button
              type="button"
              onClick={(e) => remove(e, room.id)}
              className="btn btn-sm btn-circle">
              <X className="h-6 w-6" />
            </button>
            {room.name}
          </div>
        ))}
      </div>
    </div>
  );
}
