const files = [{
    id:1,
    name: "teste",
    tags: [
        { id: 1, file: {id: 1}, tag: {id:1} },
        { id: 2, file: {id: 1}, tag: {id:2} },
    ]
}, {
    id:2,
    name: "teste2",
    tags: [
        { id: 3, file: {id: 1}, tag: {id:1} },
    ]
}]

const tags = []

// examples of what I want to get
// if tags = [1], should return file with ids 1 and 2
// if tags = [2], should return file with id 1

// const result = files.filter(file => 
//   tags.every(tag => file.tags.map(fileTag => fileTag.tag.id).includes(tag))
// );



const result = files.filter(file => 
  tags.every(tag => file.tags.some(fileTag => fileTag.tag.id === tag))
);
console.log(result)