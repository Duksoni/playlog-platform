import { ComponentFixture, TestBed } from '@angular/core/testing';

import { SimpleDialog } from './simple-dialog';

describe('SimpleDialog', () => {
  let component: SimpleDialog;
  let fixture: ComponentFixture<SimpleDialog>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [SimpleDialog]
    })
    .compileComponents();

    fixture = TestBed.createComponent(SimpleDialog);
    component = fixture.componentInstance;
    await fixture.whenStable();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
